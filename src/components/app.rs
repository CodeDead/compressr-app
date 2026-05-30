use crate::components::state::State;
use crate::services;
use crate::services::image_service::{
    CompressionParams, CompressionResult, ImageService, OutputFormat,
};
use crate::services::update_service::{UpdateInfo, UpdateService};
use crate::views::{
    about_view, error_view, main_view, no_update_view, results_view, settings_view, update_view,
};
use iced::widget::space;
use iced::window::Position;
#[cfg(target_os = "linux")]
use iced::window::settings::PlatformSpecific;
use iced::{Element, Size, Subscription, Task, Theme, clipboard, window};
use log::{error, info};
use rfd::FileDialog;
use std::collections::BTreeMap;

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
    Results,
}

#[derive(Debug, Clone)]
pub enum Message {
    MainViewOpened(window::Id),
    ViewOpened(String, WindowKind, window::Id),
    WindowClosed(window::Id),
    SelectInput,
    SelectOutput,
    SelectInputFolder,
    ToggleInputDropdown,
    DismissInputDropdown,
    Compress,
    SingleFileCompressed(Result<CompressionResult, String>),
    CloseResultsView,
    FormatSelected(OutputFormat),
    QualityChanged(u8),
    WidthChanged(i32),
    HeightChanged(i32),
    CompressionScaleChanged(u32),
    IgnoreQuality,
    IgnoreScale,
    IgnoreFormatSelected,
    IgnoreWidth,
    IgnoreHeight,
    AutoUpdateToggled(bool),
    DeleteFilesAfterCompressionToggled(bool),
    PreserveExifToggled(bool),
    ShowCompressionResultsToggled(bool),
    ThemeChanged(Theme),
    ResetSettings,
    LanguageChanged(String),
    OpenSettings,
    OpenAbout,
    OpenErrorView,
    CloseUpdateView,
    CloseErrorView,
    CloseNoUpdateView,
    CheckForUpdates(bool),
    UpdateCheckCompleted {
        result: Result<Option<UpdateInfo>, String>,
        show_no_update_view: bool,
    },
    OpenUpdateInformation,
    DownloadUpdate,
    CopyError,
    OpenCodeDeadPage,
    OpenDonationPage,
}

pub struct App {
    pub windows: BTreeMap<window::Id, Window>,
    pub state: State,
    pub image_service: ImageService,
    pub update_service: UpdateService,
    icon: window::icon::Icon,
}

impl App {
    /// Initialize a new App with an empty window map and default state, then open the main view.
    ///
    /// # Returns
    ///
    /// A tuple of the new App instance and a Task that opens the main view when executed by iced.
    pub fn new() -> (Self, Task<Message>) {
        info!("Initializing new App");

        let icon = Self::load_icon();
        let settings = Self::make_window_settings((650.0, 420.0), icon.clone());
        let (_, open) = window::open(settings);

        let state = State::default();
        let update_server = state.settings.update_server.clone();

        (
            Self {
                windows: BTreeMap::new(),
                state,
                image_service: ImageService::new(),
                update_service: UpdateService::new(update_server),
                icon,
            },
            open.map(Message::MainViewOpened),
        )
    }

    /// Returns the title of the window with the given ID, or an empty string if no such window exists.
    ///
    /// # Arguments
    ///
    /// * `window` - The ID of the window whose title is to be retrieved.
    ///
    /// # Returns
    ///
    /// The title of the window, or an empty string if no such window exists.
    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|w| w.title.clone())
            .unwrap_or_default()
    }

    /// Returns the theme of the window with the given ID, or None if no such window exists.
    ///
    /// # Arguments
    ///
    /// * `window` - The ID of the window whose theme is to be retrieved.
    ///
    /// # Returns
    ///
    /// The theme of the window, or None if no such window exists.
    pub fn theme(&self, window: window::Id) -> Option<Theme> {
        Some(self.windows.get(&window)?.theme.clone())
    }

    /// Returns the current scale factor of the window with the given ID, or 1.0 if no such window exists.
    ///
    /// # Arguments
    ///
    /// * `window` - The ID of the window whose scale factor is to be retrieved.
    ///
    /// # Returns
    ///
    /// The current scale factor of the window, or 1.0 if no such window exists.
    pub fn scale_factor(&self, window: window::Id) -> f32 {
        self.windows
            .get(&window)
            .map(|w| w.current_scale)
            .unwrap_or(1.0)
    }

    /// Subscribes to window close events and maps them to `Message::WindowClosed`.
    ///
    /// # Returns
    ///
    /// A subscription that listens for window close events and produces `Message::WindowClosed` messages when they occur.
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }

    /// Returns the view for the window with the given ID, or an empty space if no such window exists.
    ///
    /// # Arguments
    ///
    /// * `window_id` - The ID of the window whose view is to be retrieved.
    /// # Returns
    ///
    /// An `Element` representing the view for the specified window, or an empty space if no such window exists.
    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            match window.kind {
                WindowKind::Main => main_view::view(&self.state),
                WindowKind::Settings => settings_view::view(&self.state),
                WindowKind::Update => update_view::view(&self.state),
                WindowKind::Error => error_view::view(&self.state),
                WindowKind::About => about_view::view(&self.state),
                WindowKind::NoUpdate => no_update_view::view(&self.state),
                WindowKind::Results => results_view::view(&self.state),
            }
        } else {
            space().into()
        }
    }

    /// Handles incoming messages and updates the application state accordingly.
    /// Returns a `Task<Message>` that represents any asynchronous work that needs to be performed as a result of the message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be processed, which can represent user actions, lifecycle events, or other interactions within the application.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that represents any asynchronous work that needs to be performed as a result of processing the message.
    /// This can include tasks such as opening new windows, performing file I/O, checking for updates, or any other operations that should not block the main thread.
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
                    match std::fs::read_dir(&folder) {
                        Err(e) => {
                            self.state.last_error_message =
                                Some(format!("Could not read folder '{}': {e}", folder.display()));
                            self.state.compression_succeeded = false;
                            return Task::done(Message::OpenErrorView);
                        }
                        Ok(entries) => {
                            let mut entry_errors: Vec<String> = Vec::new();
                            let mut files: Vec<String> = entries
                                .filter_map(|entry| match entry {
                                    Err(e) => {
                                        entry_errors.push(format!("Directory entry error: {e}"));
                                        None
                                    }
                                    Ok(entry) => {
                                        let path = entry.path();
                                        if !path.is_file() {
                                            return None;
                                        }
                                        let ext =
                                            path.extension()?.to_string_lossy().to_lowercase();
                                        IMAGE_EXTENSIONS
                                            .contains(&ext.as_str())
                                            .then(|| path.display().to_string())
                                    }
                                })
                                .collect();

                            if !entry_errors.is_empty() {
                                self.state.last_error_message = Some(entry_errors.join("\n"));
                                self.state.compression_succeeded = false;
                                return Task::done(Message::OpenErrorView);
                            }

                            files.sort();
                            self.state.input_path = files;
                        }
                    }
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
            Message::Compress => {
                if self.state.input_path.is_empty() {
                    self.state.last_error_message = Some(
                        "No input files selected. Please select at least one file.".to_string(),
                    );
                    return Task::done(Message::OpenErrorView);
                }
                if self.state.output_path.is_empty() {
                    self.state.last_error_message = Some(
                        "No output directory selected. Please select an output directory."
                            .to_string(),
                    );
                    return Task::done(Message::OpenErrorView);
                }
                self.state.is_compressing = true;
                self.state.compression_succeeded = false;
                self.state.show_input_dropdown = false;
                self.state.compression_results = Vec::new();
                self.state.last_error_message = None;

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
                    output_path_override: None,
                };
                self.state.progress_total = input.len();
                self.state.progress_completed = 0;

                // Resolve output paths, disambiguating collisions
                let resolved_paths: Vec<String> = {
                    use std::collections::HashSet;
                    let mut seen: HashSet<String> = HashSet::new();
                    input
                        .iter()
                        .map(|file| {
                            let candidate = image_service.resolve_output_path(file, &params);
                            if seen.insert(candidate.clone()) {
                                // First time this path is used – keep it as-is.
                                candidate
                            } else {
                                // Collision: find the lowest suffix N ≥ 2 whose
                                // resulting name is not already allocated.
                                let path = std::path::PathBuf::from(&candidate);
                                let ext = path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or_default()
                                    .to_owned();
                                let stem = path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("output")
                                    .to_owned();
                                let mut n: u32 = 2;
                                loop {
                                    let new_name = if ext.is_empty() {
                                        format!("{}_{}", stem, n)
                                    } else {
                                        format!("{}_{}.{}", stem, n, ext)
                                    };
                                    let disambiguated = path
                                        .with_file_name(&new_name)
                                        .to_string_lossy()
                                        .into_owned();
                                    if seen.insert(disambiguated.clone()) {
                                        break disambiguated;
                                    }
                                    n += 1;
                                }
                            }
                        })
                        .collect()
                };

                let tasks: Vec<Task<Message>> = input
                    .into_iter()
                    .zip(resolved_paths)
                    .map(|(file, out_path)| {
                        let svc = image_service.clone();
                        let mut p = params.clone();
                        p.output_path_override = Some(out_path);
                        Task::perform(
                            tokio::task::spawn_blocking(move || svc.compress_single(file, &p)),
                            |result| match result {
                                Ok(r) => Message::SingleFileCompressed(r),
                                Err(e) => Message::SingleFileCompressed(Err(format!(
                                    "Compression task failed: {e}"
                                ))),
                            },
                        )
                    })
                    .collect();

                Task::batch(tasks)
            }
            Message::SingleFileCompressed(result) => {
                self.state.progress_completed += 1;
                match result {
                    Ok(r) => self.state.compression_results.push(r),
                    Err(e) => {
                        error!("Compression error: {e}");
                        let msg = if let Some(ref existing) = self.state.last_error_message {
                            format!("{existing}\n{e}")
                        } else {
                            e
                        };
                        self.state.last_error_message = Some(msg);
                    }
                }

                if self.state.progress_completed == self.state.progress_total {
                    self.state.is_compressing = false;

                    let has_errors = self.state.last_error_message.is_some();
                    let delete_original = self.state.settings.delete_files_after_compression;

                    if has_errors {
                        return Task::done(Message::OpenErrorView);
                    }

                    if delete_original {
                        for file in &self.state.input_path {
                            if let Err(e) = std::fs::remove_file(file) {
                                error!("Failed to delete original file '{file}': {e}");
                                self.state.last_error_message =
                                    Some(format!("Failed to delete original file '{file}': {e}"));
                                return Task::done(Message::OpenErrorView);
                            }
                        }
                    }

                    self.state.compression_succeeded = true;
                    if self.state.settings.show_compression_results {
                        let title = self.state.current_language().compressr_results.clone();
                        self.open_window(WindowKind::Results, title, (600.0, 400.0))
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }
            Message::FormatSelected(f) => {
                self.state.format = f;
                if f != OutputFormat::Jpeg && f != OutputFormat::WebP {
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
            Message::ShowCompressionResultsToggled(v) => {
                self.state.settings.show_compression_results = v;
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
            Message::OpenSettings => {
                let title = self.state.current_language().compressr_settings.clone();
                self.open_window(WindowKind::Settings, title, (500.0, 360.0))
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
            Message::CloseResultsView => self.close_window(WindowKind::Results),
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
            Message::CopyError => match self.state.last_error_message.clone() {
                Some(msg) => clipboard::write(msg),
                None => Task::none(),
            },
            Message::OpenCodeDeadPage => self.open_url_or_error("https://codedead.com/"),
            Message::OpenDonationPage => self.open_url_or_error("https://codedead.com/donate"),
        }
    }

    /// Loads the application icon from embedded PNG bytes.
    ///
    /// # Returns
    ///
    /// An `Icon` object representing the application icon, ready to be used in window settings.
    /// If the icon fails to load, the function will panic with an error message.
    fn load_icon() -> window::icon::Icon {
        let bytes = include_bytes!("../../resources/compressr.png");
        let img = image::load_from_memory(bytes).unwrap().into_rgba8();
        let (w, h) = (img.width(), img.height());
        window::icon::from_rgba(img.into_raw(), w, h).expect("Failed to load window icon")
    }

    /// Returns `"x64"`, `"aarch64"`, or `"unknown"` based on compile-time target.
    ///
    /// # Returns
    ///
    /// A string slice representing the architecture of the target platform, used for update checks and reporting.
    /// This function uses compile-time configuration to determine the architecture and returns a corresponding string.
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
    ///
    /// # Arguments
    ///
    /// * `size` - A tuple containing the width and height of the window in pixels.
    /// * `icon` - An `Icon` object to be used as the window's icon.
    ///
    /// # Returns
    ///
    /// A `window::Settings` object, configured with the specified size, icon, and other properties such as transparency, decorations, and blur.
    /// The settings also include platform-specific configuration for Linux to set the application ID.
    /// This function centralizes the window configuration to ensure consistency across different windows in the application.
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
    ///
    /// # Returns
    ///
    /// The active theme from the application settings, cloned to ensure ownership.
    fn current_theme(&self) -> Theme {
        self.state.settings.theme.clone()
    }

    /// Returns the window ID for `kind` if one is currently open.
    ///
    /// # Arguments
    ///
    /// * `kind` - The `WindowKind` to search for among the currently open windows.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `window::Id` of the first window found with the specified `WindowKind`, or `None` if no such window is currently open.
    fn find_window_by_kind(&self, kind: WindowKind) -> Option<window::Id> {
        self.windows
            .iter()
            .find(|(_, w)| w.kind == kind)
            .map(|(id, _)| *id)
    }

    /// Opens a window of `kind` unless one is already open.
    /// Positions the new window relative to the last existing window.
    ///
    /// # Arguments
    ///
    /// * `kind` - The `WindowKind` representing the type of window to open (e.g., Settings, About, Error).
    /// * `title` - The title to display in the window's title bar.
    /// * `size` - A tuple specifying the width and height of the window in pixels.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that, when executed, will open a new window of the specified kind with the given title and size, positioned relative to the last existing window.
    /// If a window of the specified kind is already open, the function returns a no-op task that does nothing when executed
    fn open_window(&self, kind: WindowKind, title: String, size: (f32, f32)) -> Task<Message> {
        let Some(last) = self.windows.keys().last() else {
            return Task::none();
        };
        if self.windows.values().any(|w| w.kind == kind) {
            return Task::none();
        }
        let icon = self.icon.clone();
        window::position(*last)
            .then(move |_| {
                let (_, open) = window::open(Self::make_window_settings(size, icon.clone()));
                open
            })
            .map(move |id| Message::ViewOpened(title.clone(), kind, id))
    }

    /// Removes `kind` from the window map and sends `window::close`.
    ///
    /// # Arguments
    ///
    /// * `kind` - The `WindowKind` representing the type of window to close (e.g., Settings, About, Error).
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that, when executed, will close the window of the specified kind if it is currently open, and remove it from the application's window map.
    /// If no window of the specified kind is currently open, the function returns a no-op task that does nothing when executed.
    fn close_window(&mut self, kind: WindowKind) -> Task<Message> {
        if let Some(id) = self.find_window_by_kind(kind) {
            self.windows.remove(&id);
            return window::close(id);
        }
        Task::none()
    }

    /// Opens `url` in the default browser, routing any error to the error view.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to open in the default web browser.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that, when executed, will attempt to open the specified URL in the user's default web browser.
    /// If the operation is successful, it logs the action and does nothing further.
    /// If an error occurs while trying to open the URL, it logs the error and updates the application's state with the error message.
    /// It returns a task that will trigger the opening of the error view to inform the user about the issue.
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
    ///
    /// # Arguments
    ///
    /// * `show_no_update_view` - A boolean flag indicating whether to show a "No Update" view if no updates are found. If `true`, the application will display a view informing the user that they are using the latest version. If `false`, the application will simply do nothing if no updates are available.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that, when executed, will perform an asynchronous check for updates using the application's `UpdateService`.
    /// The task will gather necessary information such as the current version, platform, and architecture, and then call the update service to check for available updates.
    /// Once the check is completed, it will produce a `Message::UpdateCheckCompleted` message containing the result of the update check and the `show_no_update_view` flag,
    /// which can be used by the application to determine how to respond to the update check results (e.g., whether to display a "No Update" view).
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

impl Window {
    /// Creates a new `Window` instance with the specified title, kind, and theme.
    ///
    /// # Arguments
    ///
    /// * `title` - A `String` representing the title of the window, which will be displayed in the window's title bar.
    /// * `kind` - A `WindowKind` enum value that categorizes the type of window (e.g., Main, Settings, Update, Error, About, NoUpdate, Results). This is used to manage different windows within the application and determine their behavior and content.
    /// * `theme` - A `Theme` object that specifies the visual theme of the window (e.g., light, dark, or custom). This allows the window to be styled according to the user's preferences or the application's design.
    ///
    /// # Returns
    ///
    /// A new instance of the `Window` struct, initialized with the provided title, kind, and theme.
    /// The `current_scale` field is set to a default value of `1.0`, indicating that the window is initially displayed at its normal scale.
    fn new(title: String, kind: WindowKind, theme: Theme) -> Self {
        Self {
            title,
            current_scale: 1.0,
            theme,
            kind,
        }
    }
}
