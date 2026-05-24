use crate::components::state::State;
use crate::services;
use crate::services::image_service::{CompressionParams, ImageService, OutputFormat};
use crate::services::update_service::{UpdateInfo, UpdateService};
use crate::views::{about_view, error_view, main_view, no_update_view, settings_view, update_view};
use iced::widget::space;
use iced::window::Position;
#[cfg(target_os = "linux")]
use iced::window::settings::PlatformSpecific;
use iced::{Element, Size, Subscription, Task, Theme, clipboard, window};
use log::{error, info};
use rfd::FileDialog;
use std::collections::BTreeMap;

// ── Window ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Window {
    title: String,
    current_scale: f32,
    theme: Theme,
    kind: WindowKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Main,
    Settings,
    Update,
    Error,
    About,
    NoUpdate,
}

// ── Message ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    // Lifecycle
    MainViewOpened(window::Id),
    ViewOpened(String, WindowKind, window::Id),
    WindowClosed(window::Id),
    // Input selection
    SelectInput,
    SelectOutput,
    SelectInputFolder,
    ToggleInputDropdown,
    DismissInputDropdown,
    // Compression
    Compress,
    CompressionCompleted(Result<(), String>),
    // Compression parameters
    FormatSelected(OutputFormat),
    QualityChanged(u8),
    WidthChanged(i32),
    HeightChanged(i32),
    CompressionScaleChanged(u32),
    // Disabled-state no-ops (callbacks required by widget API but values ignored)
    IgnoreQuality,
    IgnoreScale,
    IgnoreFormatSelected,
    IgnoreWidth,
    IgnoreHeight,
    // Settings
    AutoUpdateToggled(bool),
    DeleteFilesAfterCompressionToggled(bool),
    PreserveExifToggled(bool),
    ThemeChanged(Theme),
    ResetSettings,
    LanguageChanged(String),
    // Windows
    OpenSettings,
    OpenAbout,
    OpenErrorView,
    CloseUpdateView,
    CloseErrorView,
    CloseNoUpdateView,
    // Updates
    CheckForUpdates(bool),
    UpdateCheckCompleted {
        result: Result<Option<UpdateInfo>, String>,
        show_no_update_view: bool,
    },
    OpenUpdateInformation,
    DownloadUpdate,
    // Misc
    CopyError,
    OpenCodeDeadPage,
    OpenDonationPage,
}

// ── App ───────────────────────────────────────────────────────────────────────

pub struct App {
    pub windows: BTreeMap<window::Id, Window>,
    pub state: State,
    pub image_service: ImageService,
    pub update_service: UpdateService,
}

impl App {
    // ── Initialisation ────────────────────────────────────────────────────────

    pub fn new() -> (Self, Task<Message>) {
        info!("Initializing new App");

        let icon = Self::load_icon();
        let settings = Self::make_window_settings((650.0, 420.0), icon);
        let (_, open) = window::open(settings);

        let state = State::default();
        let update_server = state.settings.update_server.clone();

        (
            Self {
                windows: BTreeMap::new(),
                state,
                image_service: ImageService::new(),
                update_service: UpdateService::new(update_server),
            },
            open.map(Message::MainViewOpened),
        )
    }

    // ── iced entry-points ─────────────────────────────────────────────────────

    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|w| w.title.clone())
            .unwrap_or_default()
    }

    pub fn theme(&self, window: window::Id) -> Option<Theme> {
        Some(self.windows.get(&window)?.theme.clone())
    }

    pub fn scale_factor(&self, window: window::Id) -> f32 {
        self.windows
            .get(&window)
            .map(|w| w.current_scale)
            .unwrap_or(1.0)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            match window.kind {
                WindowKind::Main => main_view::view(&self.state),
                WindowKind::Settings => settings_view::view(&self.state),
                WindowKind::Update => update_view::view(&self.state),
                WindowKind::Error => error_view::view(&self.state),
                WindowKind::About => about_view::view(&self.state),
                WindowKind::NoUpdate => no_update_view::view(&self.state),
            }
        } else {
            space().into()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // ── Lifecycle ─────────────────────────────────────────────────────
            Message::MainViewOpened(id) => {
                self.windows.insert(
                    id,
                    Window::new(
                        "Compressr".to_string(),
                        WindowKind::Main,
                        self.current_theme(),
                    ),
                );
                if self.state.settings.auto_update {
                    return self.spawn_update_check(false);
                }
                Task::none()
            }
            Message::ViewOpened(title, kind, id) => {
                self.windows
                    .insert(id, Window::new(title, kind, self.current_theme()));
                Task::none()
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }

            // ── Input selection ───────────────────────────────────────────────
            Message::SelectInput => {
                self.state.show_input_dropdown = false;
                if let Some(paths) = FileDialog::new()
                    .add_filter(
                        "Image files",
                        &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tiff"],
                    )
                    .pick_files()
                {
                    self.state.input_path = paths.iter().map(|p| p.display().to_string()).collect();
                }
                self.state.compression_succeeded = false;
                Task::none()
            }
            Message::SelectOutput => {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.state.output_path = path.display().to_string();
                }
                Task::none()
            }
            Message::SelectInputFolder => {
                self.state.show_input_dropdown = false;
                if let Some(folder) = FileDialog::new().pick_folder() {
                    const IMAGE_EXTENSIONS: &[&str] =
                        &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tiff"];
                    let mut files: Vec<String> = std::fs::read_dir(&folder)
                        .into_iter()
                        .flatten()
                        .filter_map(|entry| entry.ok())
                        .filter_map(|entry| {
                            let path = entry.path();
                            if !path.is_file() {
                                return None;
                            }
                            let ext = path.extension()?.to_string_lossy().to_lowercase();
                            IMAGE_EXTENSIONS
                                .contains(&ext.as_str())
                                .then(|| path.display().to_string())
                        })
                        .collect();
                    files.sort();
                    self.state.input_path = files;
                }
                self.state.compression_succeeded = false;
                Task::none()
            }
            Message::ToggleInputDropdown => {
                self.state.show_input_dropdown = !self.state.show_input_dropdown;
                Task::none()
            }
            Message::DismissInputDropdown => {
                self.state.show_input_dropdown = false;
                Task::none()
            }

            // ── Compression ───────────────────────────────────────────────────
            Message::Compress => {
                self.state.is_compressing = true;
                self.state.compression_succeeded = false;

                let input = self.state.input_path.clone();
                let output = self.state.output_path.clone();
                let image_service = self.image_service.clone();
                let params = CompressionParams {
                    output_path: output.clone(),
                    is_output_a_directory: std::fs::metadata(&output)
                        .map(|m| m.is_dir())
                        .unwrap_or(false),
                    scale: self.state.scale,
                    width: self.state.width,
                    height: self.state.height,
                    quality: self.state.quality,
                    format: self.state.format,
                    preserve_exif: self.state.settings.preserve_exif,
                };
                let delete_original = self.state.settings.delete_files_after_compression;

                Task::perform(
                    compress_images(input, params, image_service, delete_original),
                    Message::CompressionCompleted,
                )
            }
            Message::CompressionCompleted(result) => {
                self.state.is_compressing = false;
                match result {
                    Ok(()) => {
                        self.state.compression_succeeded = true;
                        Task::none()
                    }
                    Err(err) => {
                        error!("Compression failed: {err}");
                        self.state.last_error_message = Some(err);
                        Task::done(Message::OpenErrorView)
                    }
                }
            }

            // ── Compression parameters ────────────────────────────────────────
            Message::FormatSelected(f) => {
                self.state.format = f;
                if f != OutputFormat::Jpeg {
                    self.state.quality = 100;
                }
                Task::none()
            }
            Message::QualityChanged(q) => {
                self.state.quality = q;
                Task::none()
            }
            Message::WidthChanged(w) => {
                self.state.width = (w > 0).then_some(w as u32);
                Task::none()
            }
            Message::HeightChanged(h) => {
                self.state.height = (h > 0).then_some(h as u32);
                Task::none()
            }
            Message::CompressionScaleChanged(s) => {
                self.state.scale = s;
                Task::none()
            }
            Message::IgnoreQuality
            | Message::IgnoreScale
            | Message::IgnoreFormatSelected
            | Message::IgnoreWidth
            | Message::IgnoreHeight => Task::none(),

            // ── Settings ──────────────────────────────────────────────────────
            Message::AutoUpdateToggled(v) => {
                self.state.settings.auto_update = v;
                self.state.settings.save();
                Task::none()
            }
            Message::DeleteFilesAfterCompressionToggled(v) => {
                self.state.settings.delete_files_after_compression = v;
                self.state.settings.save();
                Task::none()
            }
            Message::PreserveExifToggled(v) => {
                self.state.settings.preserve_exif = v;
                self.state.settings.save();
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.state.settings.theme = theme.clone();
                self.state.settings.save();
                self.windows
                    .values_mut()
                    .for_each(|w| w.theme = theme.clone());
                Task::none()
            }
            Message::ResetSettings => {
                self.state.settings = crate::components::settings::Settings::default();
                self.update_service = UpdateService::new(self.state.settings.update_server.clone());
                let theme = self.current_theme();
                self.windows
                    .values_mut()
                    .for_each(|w| w.theme = theme.clone());
                Task::none()
            }
            Message::LanguageChanged(new_language) => {
                let key = self
                    .state
                    .languages
                    .iter()
                    .find(|l| l.language_name == new_language)
                    .unwrap_or(&self.state.languages[0])
                    .language_key
                    .clone();
                self.state.settings.language_key = key;
                self.state.settings.save();
                Task::none()
            }

            // ── Window management ─────────────────────────────────────────────
            Message::OpenSettings => {
                let title = self.state.current_language().compressr_settings.clone();
                self.open_window(WindowKind::Settings, title, (500.0, 325.0))
            }
            Message::OpenAbout => {
                let title = self.state.current_language().compressr_about.clone();
                self.open_window(WindowKind::About, title, (450.0, 270.0))
            }
            Message::OpenErrorView => {
                let title = self.state.current_language().compressr_error.clone();
                self.open_window(WindowKind::Error, title, (400.0, 210.0))
            }
            Message::CloseUpdateView => self.close_window(WindowKind::Update),
            Message::CloseErrorView => self.close_window(WindowKind::Error),
            Message::CloseNoUpdateView => self.close_window(WindowKind::NoUpdate),

            // ── Updates ───────────────────────────────────────────────────────
            Message::CheckForUpdates(show_no_update_view) => {
                self.spawn_update_check(show_no_update_view)
            }
            Message::UpdateCheckCompleted {
                result,
                show_no_update_view,
            } => match result {
                Ok(Some(update_info)) => {
                    info!("Update available: {}", update_info.semver);
                    self.state.last_error_message = None;
                    self.state.update_version = Some(update_info.semver);
                    self.state.update_download_url = Some(update_info.download_url);
                    self.state.update_info_url = update_info.info_url;

                    let title = self.state.current_language().compressr_update.clone();
                    self.open_window(WindowKind::Update, title, (400.0, 190.0))
                }
                Ok(None) => {
                    info!("No updates available");
                    self.state.last_error_message = None;
                    self.state.update_version = None;
                    self.state.update_download_url = None;
                    self.state.update_info_url = None;

                    if !show_no_update_view {
                        return Task::none();
                    }
                    let title = self.state.current_language().compressr_update.clone();
                    self.open_window(WindowKind::NoUpdate, title, (400.0, 180.0))
                }
                Err(err) => {
                    error!("Failed to check for updates: {err}");
                    self.state.last_error_message = Some(err);
                    Task::done(Message::OpenErrorView)
                }
            },
            Message::OpenUpdateInformation => {
                let url = self
                    .state
                    .update_info_url
                    .clone()
                    .unwrap_or_else(|| "https://codedead.com/".to_string());
                self.open_url_or_error(&url)
            }
            Message::DownloadUpdate => {
                let url = self
                    .state
                    .update_download_url
                    .clone()
                    .unwrap_or_else(|| "https://codedead.com/".to_string());
                match services::open_website(&url) {
                    Ok(_) => {
                        info!("Opened download URL; exiting");
                        iced::exit()
                    }
                    Err(err) => {
                        self.state.last_error_message = Some(err);
                        Task::done(Message::OpenErrorView)
                    }
                }
            }

            // ── Misc ──────────────────────────────────────────────────────────
            Message::CopyError => match self.state.last_error_message.clone() {
                Some(msg) => clipboard::write(msg),
                None => Task::none(),
            },
            Message::OpenCodeDeadPage => self.open_url_or_error("https://codedead.com/"),
            Message::OpenDonationPage => self.open_url_or_error("https://codedead.com/donate"),
        }
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Loads the application icon from embedded PNG bytes.
    fn load_icon() -> window::icon::Icon {
        let bytes = include_bytes!("../../resources/compressr.png");
        let img = image::load_from_memory(bytes).unwrap().into_rgba8();
        let (w, h) = (img.width(), img.height());
        window::icon::from_rgba(img.into_raw(), w, h).expect("Failed to load window icon")
    }

    /// Returns `"x64"`, `"aarch64"`, or `"unknown"` based on compile-time target.
    fn arch() -> &'static str {
        if cfg!(target_arch = "x86_64") {
            "x64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "unknown"
        }
    }

    /// Builds [`window::Settings`] for the given pixel size and icon.
    fn make_window_settings(size: (f32, f32), icon: window::icon::Icon) -> window::Settings {
        window::Settings {
            size: Size::new(size.0, size.1),
            resizable: true,
            position: Position::Centered,
            transparent: true,
            decorations: true,
            blur: true,
            icon: Some(icon),
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: "com.codedead.compressr".to_string(),
                ..PlatformSpecific::default()
            },
            ..window::Settings::default()
        }
    }

    /// Returns the active [`Theme`] from settings.
    fn current_theme(&self) -> Theme {
        self.state.settings.theme.clone()
    }

    /// Returns the window ID for `kind`, if one is currently open.
    fn find_window_by_kind(&self, kind: WindowKind) -> Option<window::Id> {
        self.windows
            .iter()
            .find(|(_, w)| w.kind == kind)
            .map(|(id, _)| *id)
    }

    /// Opens a window of `kind` unless one is already open.
    /// Positions the new window relative to the last existing window.
    fn open_window(&self, kind: WindowKind, title: String, size: (f32, f32)) -> Task<Message> {
        let Some(last) = self.windows.keys().last() else {
            return Task::none();
        };
        if self.windows.values().any(|w| w.kind == kind) {
            return Task::none();
        }
        window::position(*last)
            .then(move |_| {
                let (_, open) = window::open(Self::make_window_settings(size, Self::load_icon()));
                open
            })
            .map(move |id| Message::ViewOpened(title.clone(), kind, id))
    }

    /// Removes `kind` from the window map and sends `window::close`.
    fn close_window(&mut self, kind: WindowKind) -> Task<Message> {
        if let Some(id) = self.find_window_by_kind(kind) {
            self.windows.remove(&id);
            return window::close(id);
        }
        Task::none()
    }

    /// Opens `url` in the default browser, routing any error to the error view.
    fn open_url_or_error(&mut self, url: &str) -> Task<Message> {
        match services::open_website(url) {
            Ok(_) => {
                info!("Opened URL: {url}");
                Task::none()
            }
            Err(err) => {
                error!("Failed to open URL '{url}': {err}");
                self.state.last_error_message = Some(err);
                Task::done(Message::OpenErrorView)
            }
        }
    }

    /// Builds a [`Task`] that checks for updates and reports results via
    /// [`Message::UpdateCheckCompleted`].
    fn spawn_update_check(&self, show_no_update_view: bool) -> Task<Message> {
        let semver = env!("CARGO_PKG_VERSION").to_string();
        let platform = crate::get_platform().to_string();
        let arch = Self::arch().to_string();
        let svc = self.update_service.clone();
        Task::perform(
            async move { svc.check_for_updates(semver, platform, arch).await },
            move |result| Message::UpdateCheckCompleted {
                result,
                show_no_update_view,
            },
        )
    }
}

// ── Window ────────────────────────────────────────────────────────────────────

impl Window {
    fn new(title: String, kind: WindowKind, theme: Theme) -> Self {
        Self {
            title,
            current_scale: 1.0,
            theme,
            kind,
        }
    }
}

// ── Free async helpers ────────────────────────────────────────────────────────

/// Compresses all `input` files concurrently, then optionally deletes the originals.
/// Extracted so the `update()` match arm stays readable.
async fn compress_images(
    input: Vec<String>,
    params: CompressionParams,
    image_service: ImageService,
    delete_original: bool,
) -> Result<(), String> {
    if input.is_empty() {
        return Err("Input path cannot be empty".to_string());
    }
    if params.output_path.is_empty() {
        return Err("Output path cannot be empty".to_string());
    }

    // Compress every file concurrently on the blocking thread pool.
    let handles: Vec<tokio::task::JoinHandle<Result<(), String>>> = input
        .iter()
        .map(|file| {
            let file = file.clone();
            let svc = image_service.clone();
            let p = params.clone();
            tokio::task::spawn_blocking(move || svc.compress_single(file, &p))
        })
        .collect();

    let errors: Vec<String> = {
        let mut errs = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(())) => {}
                Ok(Err(e)) => errs.push(e),
                Err(e) => errs.push(format!("Compression task panicked: {e}")),
            }
        }
        errs
    };

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    if delete_original {
        for file in &input {
            std::fs::remove_file(file)
                .map_err(|e| format!("Failed to delete original file '{file}': {e}"))?;
        }
    }

    Ok(())
}
