use crate::components::state::State;
use crate::components::window::{Window, WindowKind, load_app_icon, make_window_settings};
use crate::services;
use crate::services::folder_scanner::{IMAGE_EXTENSIONS, scan_folder};
use crate::services::image_service::{
    CompressionParams, CompressionResult, ImageService, OutputFormat,
};
use crate::services::update_service::{UpdateInfo, UpdateService};
use iced::widget::space;
use iced::{Element, Subscription, Task, Theme, clipboard, window};
use log::{error, info};
use rfd::FileDialog;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

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
    InputFolderScanCompleted(Vec<String>),
    InputFolderScanFailed(String),
    FormatSelected(OutputFormat),
    QualityChanged(u8),
    WidthChanged(i32),
    HeightChanged(i32),
    CompressionScaleChanged(u32),
    Noop,
    AutoUpdateToggled(bool),
    DeleteFilesAfterCompressionToggled(bool),
    PreserveExifToggled(bool),
    ShowCompressionResultsToggled(bool),
    RecursiveFolderScanToggled(bool),
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

macro_rules! settings_toggle {
    ($app:expr, $field:ident, $value:expr) => {{
        $app.state.settings.$field = $value;
        $app.handle_settings_save_result($app.state.settings.save())
    }};
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

        let icon = load_app_icon();
        let settings = make_window_settings(WindowKind::Main.default_size(), icon.clone());
        let (_, open) = window::open(settings);

        let (loaded_settings, settings_loaded_from_file) =
            crate::components::settings::Settings::load_from_file();
        let mut state = State::with_settings(loaded_settings);
        let update_server = state.settings.update_server.clone();

        // Only persist to disk when settings were NOT loaded from an existing file
        // (first run, missing file, or corrupt config). Skips the redundant write
        // on normal startups where the config was read successfully.
        if !settings_loaded_from_file && let Err(e) = state.settings.save() {
            error!("Failed to persist initial settings: {e}");
            state.last_error_message = Some(e.to_string());
        }

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
    pub fn scale_factor(&self, _window: window::Id) -> f32 {
        1.0
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
            window.kind.view(&self.state)
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
                        WindowKind::Main.title(self.state.current_language()),
                        WindowKind::Main,
                        self.current_theme(),
                    ),
                );

                // Show any pending startup error now that the main window exists
                // for the error window to position relative to.
                if self.state.last_error_message.is_some() {
                    return self.open_window(WindowKind::Error);
                }

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
                let was_main = self
                    .windows
                    .get(&id)
                    .is_some_and(|w| w.kind == WindowKind::Main);
                let was_main_compressing = was_main && self.state.is_compressing;
                let was_error = self
                    .windows
                    .get(&id)
                    .is_some_and(|w| w.kind == WindowKind::Error);
                self.windows.remove(&id);
                if was_main_compressing {
                    info!("Main compressing window closed. Exiting.");
                    self.state
                        .compression_aborted
                        .store(true, Ordering::Relaxed);
                }
                if was_error {
                    self.state.last_error_message = None;
                }
                if was_main || self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::SelectInput => {
                self.state.show_input_dropdown = false;
                if let Some(paths) = FileDialog::new()
                    .add_filter("Image files", IMAGE_EXTENSIONS)
                    .pick_files()
                {
                    self.state.input_path = paths.iter().map(|p| p.display().to_string()).collect();
                }
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
                    let recursive = self.state.settings.recursive_folder_scan;
                    return Task::perform(
                        tokio::task::spawn_blocking(move || scan_folder(folder, recursive)),
                        |result| match result {
                            Ok(Ok(files)) => Message::InputFolderScanCompleted(files),
                            Ok(Err(e)) => Message::InputFolderScanFailed(e),
                            Err(e) => Message::InputFolderScanFailed(format!(
                                "Folder scan task failed: {e}"
                            )),
                        },
                    );
                }
                Task::none()
            }
            Message::InputFolderScanCompleted(files) => {
                self.state.input_path = files;
                Task::none()
            }
            Message::InputFolderScanFailed(errors) => self.error(errors),
            Message::Compress => {
                if let Err(msg) = self.validate_compression_inputs() {
                    return self.error(msg);
                }

                self.state.is_compressing = true;
                self.state
                    .compression_aborted
                    .store(false, Ordering::Relaxed);
                self.state.show_input_dropdown = false;
                self.state.compression_results = Vec::new();
                self.state.last_error_message = None;

                let input = self.state.input_path.clone();
                let params = CompressionParams {
                    output_path: Arc::from(self.state.output_path.as_str()),
                    is_output_a_directory: true,
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

                self.spawn_compression_tasks(input, params)
            }
            Message::ToggleInputDropdown => {
                self.state.show_input_dropdown = !self.state.show_input_dropdown;
                Task::none()
            }
            Message::DismissInputDropdown => {
                self.state.show_input_dropdown = false;
                Task::none()
            }
            Message::SingleFileCompressed(result) => {
                if self.state.compression_aborted.load(Ordering::Relaxed) {
                    return Task::none();
                }
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
                        self.set_error(msg);
                    }
                }

                if self.state.progress_completed == self.state.progress_total {
                    self.state.is_compressing = false;
                    return self.on_compression_complete();
                }
                Task::none()
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
            Message::Noop => Task::none(),
            Message::AutoUpdateToggled(v) => settings_toggle!(self, auto_update, v),
            Message::DeleteFilesAfterCompressionToggled(v) => {
                settings_toggle!(self, delete_files_after_compression, v)
            }
            Message::PreserveExifToggled(v) => settings_toggle!(self, preserve_exif, v),
            Message::ShowCompressionResultsToggled(v) => {
                settings_toggle!(self, show_compression_results, v)
            }
            Message::RecursiveFolderScanToggled(v) => {
                settings_toggle!(self, recursive_folder_scan, v)
            }
            Message::ThemeChanged(theme) => {
                self.state.settings.theme = theme.clone();
                self.windows
                    .values_mut()
                    .for_each(|w| w.theme = theme.clone());
                self.handle_settings_save_result(self.state.settings.save())
            }
            Message::ResetSettings => {
                self.state.settings = crate::components::settings::Settings::default();
                self.update_service = UpdateService::new(self.state.settings.update_server.clone());
                let theme = self.current_theme();
                self.windows
                    .values_mut()
                    .for_each(|w| w.theme = theme.clone());
                self.handle_settings_save_result(self.state.settings.save())
            }
            Message::LanguageChanged(new_language) => {
                let key = self
                    .state
                    .languages
                    .iter()
                    .find(|l| l.language_name == new_language)
                    .unwrap_or(
                        self.state
                            .languages
                            .first()
                            .expect("At least one language should be defined!"),
                    )
                    .language_key
                    .clone();
                self.state.settings.language_key = key;
                self.handle_settings_save_result(self.state.settings.save())
            }
            Message::OpenSettings => self.open_window(WindowKind::Settings),
            Message::OpenAbout => self.open_window(WindowKind::About),
            Message::OpenErrorView => self.open_window(WindowKind::Error),
            Message::CloseUpdateView => self.close_window(WindowKind::Update),
            Message::CloseErrorView => {
                // Reset the error message to none
                self.state.last_error_message = None;
                self.close_window(WindowKind::Error)
            }
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

                    self.open_window(WindowKind::Update)
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
                    self.open_window(WindowKind::NoUpdate)
                }
                Err(err) => {
                    error!("Failed to check for updates: {err}");
                    self.error(err)
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
                    Err(err) => self.error(err),
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
    /// The window's title and size are derived from `kind` via
    /// [`WindowKind::title`] and [`WindowKind::default_size`].
    ///
    /// # Arguments
    ///
    /// * `kind` - The `WindowKind` representing the type of window to open (e.g., Settings, About, Error).
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that, when executed, will open a new window of the specified kind, positioned relative to the last existing window.
    /// If a window of the specified kind is already open, the function returns a no-op task that does nothing when executed
    fn open_window(&self, kind: WindowKind) -> Task<Message> {
        let Some(last) = self.windows.keys().last() else {
            return Task::none();
        };
        if self.windows.values().any(|w| w.kind == kind) {
            return Task::none();
        }
        let title = kind.title(self.state.current_language());
        let size = kind.default_size();
        let icon = self.icon.clone();
        window::position(*last)
            .then(move |_| {
                let (_, open) = window::open(make_window_settings(size, icon.clone()));
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
                self.error(err)
            }
        }
    }

    /// Stores a one-shot error message without opening the error view.
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message to store.
    fn set_error(&mut self, msg: impl Into<String>) {
        self.state.last_error_message = Some(msg.into());
    }

    /// Stores an error message and returns a task that opens the error view.
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message to store.
    ///
    /// # Returns
    ///
    /// A task that opens the error view.
    fn error(&mut self, msg: impl Into<String>) -> Task<Message> {
        self.set_error(msg);
        Task::done(Message::OpenErrorView)
    }

    /// Validates that compression inputs are present and the output directory exists.
    ///
    /// # Returns
    ///
    /// A result indicating whether the inputs are valid or an error message.
    fn validate_compression_inputs(&self) -> Result<(), String> {
        if self.state.input_path.is_empty() {
            return Err("No input files selected. Please select at least one file.".to_string());
        }
        if self.state.output_path.is_empty() {
            return Err(
                "No output directory selected. Please select an output directory.".to_string(),
            );
        }
        match std::fs::metadata(&self.state.output_path) {
            Ok(m) if m.is_dir() => Ok(()),
            Ok(_) => Err(
                "Output path is a file, not a directory. Please select a directory.".to_string(),
            ),
            Err(_) => Err(format!(
                "Output directory '{}' does not exist.",
                self.state.output_path
            )),
        }
    }

    /// Builds a batched [`Task`] that compresses every input file off the GUI thread.
    ///
    /// Output paths are pre-resolved and de-duplicated up front via
    /// [`ImageService::resolve_unique_output_paths`], then each file is compressed in its own
    /// `spawn_blocking` task. Results are reported back through [`Message::SingleFileCompressed`].
    ///
    /// # Arguments
    ///
    /// * `input` - The input file paths to compress.
    /// * `params` - The compression parameters shared across the batch.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` batching one compression task per input file.
    fn spawn_compression_tasks(
        &self,
        input: Vec<String>,
        params: CompressionParams,
    ) -> Task<Message> {
        let resolved_paths = self
            .image_service
            .resolve_unique_output_paths(&input, &params);

        let tasks: Vec<Task<Message>> = input
            .into_iter()
            .zip(resolved_paths)
            .map(|(file, out_path)| {
                let svc = self.image_service.clone();
                let mut p = params.clone();
                let cancelled = Arc::clone(&self.state.compression_aborted);
                p.output_path_override = Some(out_path);
                Task::perform(
                    tokio::task::spawn_blocking(move || svc.compress_single(file, &p, cancelled)),
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

    /// Handles the post-compression completion: error display, original file deletion, and results view.
    ///
    /// # Returns
    ///
    /// A task that manages the post-compression actions.
    fn on_compression_complete(&mut self) -> Task<Message> {
        let has_errors = self.state.last_error_message.is_some();
        if has_errors {
            return Task::done(Message::OpenErrorView);
        }

        if self.state.settings.delete_files_after_compression {
            for file in &self.state.input_path {
                if let Err(e) = std::fs::remove_file(file) {
                    error!("Failed to delete original file '{file}': {e}");
                    return self.error(format!("Failed to delete original file '{file}': {e}"));
                }
            }
        }

        if self.state.settings.show_compression_results {
            self.open_window(WindowKind::Results)
        } else {
            Task::none()
        }
    }

    /// Routes a [`Settings::save`](crate::components::settings::Settings::save) result to the error view on failure.
    ///
    /// # Arguments
    ///
    /// * `result` - The result of a settings save attempt.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that, when executed, will open the error view if the save failed, or do nothing if it succeeded.
    /// If the operation fails, the error is logged and stored in the application state for display in the error view.
    fn handle_settings_save_result(&mut self, result: Result<(), std::io::Error>) -> Task<Message> {
        match result {
            Ok(()) => Task::none(),
            Err(err) => {
                error!("Failed to save settings: {err}");
                self.error(err.to_string())
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
        let svc = self.update_service.clone();
        Task::perform(
            async move { svc.check_for_updates(semver).await },
            move |result| Message::UpdateCheckCompleted {
                result,
                show_no_update_view,
            },
        )
    }
}
