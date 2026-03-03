use crate::components::state::State;
use crate::services::image_service::{ImageService, OutputFormat};
use crate::services::theme_service::ThemeService;
use crate::views::{main_view, settings_view};
use iced::widget::space;
use iced::window::Position;
#[cfg(target_os = "linux")]
use iced::window::settings::PlatformSpecific;
use iced::{Element, Size, Subscription, Task, Theme, window};
use log::{error, info};
use rfd::FileDialog;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Window {
    title: String,
    current_scale: f32,
    theme: Theme,
    window_id: u8,
}

#[derive(Debug, Clone)]
pub enum Message {
    MainViewOpened(window::Id),
    SettingsViewOpened(window::Id),
    WindowClosed(window::Id),
    SelectInput,
    SelectOutput,
    Compress,
    OpenSettings,
    CompressionCompleted(Result<(), String>),
    FormatSelected(OutputFormat),
    QualityChanged(u8),
    WidthChanged(String),
    HeightChanged(String),
    CompressionScaleChanged(u32),
    IgnoreQuality(u8),
    IgnoreScale(u32),
    IgnoreFormatSelected(OutputFormat),
    AutoUpdateToggled(bool),
    DeleteFilesAfterCompressionToggled(bool),
    ThemeChanged(Theme),
    ClearStatus,
    ResetSettings,
}

pub struct App {
    pub windows: BTreeMap<window::Id, Window>,
    pub state: State,
    pub image_service: ImageService,
}

impl App {
    /// Initialize a new instance of the application, returning both the app and an initial task to open the main view.
    ///
    /// This method sets up the application window with specific settings, including size, transparency, and an icon. It also initializes the application state and image service.
    ///
    /// Returns:
    ///
    /// - `Self`: A new instance of the `App` struct, initialized with default state and an empty window map.
    /// - `Task<Message>`: A task that, when executed, will open the main view of the application and send a `Message::MainViewOpened` with the window ID.
    ///
    /// # Panics
    ///
    /// This function will panic if the application icon cannot be loaded from the specified bytes.
    pub fn new() -> (Self, Task<Message>) {
        info!("Initializing new App");

        let icon_bytes = include_bytes!("../../resources/compressr.png");
        let image = image::load_from_memory(icon_bytes).unwrap();
        let window_icon = window::icon::from_rgba(image.as_bytes().to_vec(), 256, 256)
            .expect("Failed to load window icon");

        let settings = window::Settings {
            size: Size::new(650.0, 425.0),
            resizable: true,
            position: Position::Centered,
            transparent: true,
            decorations: true,
            blur: true,
            icon: Some(window_icon),
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: "com.codedead.compressr".to_string(),
                ..PlatformSpecific::default()
            },
            ..window::Settings::default()
        };

        let (_, open) = window::open(settings);
        (
            Self {
                windows: BTreeMap::new(),
                state: State::default(),
                image_service: ImageService::new(),
            },
            open.map(Message::MainViewOpened),
        )
    }

    /// Retrieves the title of the window with the given ID.
    ///
    /// # Arguments
    ///
    /// * `window` - The ID of the window for which to retrieve the title.
    ///
    /// # Returns
    ///
    /// The title of the window if it exists, or an empty string if the window ID is not found in the `windows` map.
    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    /// Handles incoming messages and updates the application state accordingly.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to process, which can represent various user actions or events in the application.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that represents any asynchronous operations that need to be performed as a result of processing the message. This can include tasks like opening new windows, performing file operations, or updating the UI after a delay.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainViewOpened(id) => {
                let window = Window::new(
                    "Compressr".to_string(),
                    0,
                    ThemeService::string_to_theme(
                        &self
                            .state
                            .settings
                            .theme
                            .clone()
                            .unwrap_or(Theme::Oxocarbon.to_string()),
                    ),
                );

                self.windows.insert(id, window);
                Task::none()
            }
            Message::SettingsViewOpened(id) => {
                let window = Window::new(
                    "Compressr - Settings".to_string(),
                    1,
                    ThemeService::string_to_theme(
                        &self
                            .state
                            .settings
                            .theme
                            .clone()
                            .unwrap_or(Theme::Oxocarbon.to_string()),
                    ),
                );

                self.windows.insert(id, window);
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
            Message::SelectInput => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.state.input_path = path.display().to_string();
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
            Message::Compress => {
                self.state.is_compressing = true;
                self.state.compression_succeeded = false;
                self.state.status.clear();

                let input = self.state.input_path.clone();
                let output = self.state.output_path.clone();
                let scale = self.state.scale;
                let width = self.state.width;
                let height = self.state.height;
                let quality = self.state.quality;
                let format = self.state.format;
                let image_service = self.image_service.clone();
                let delete_original = self.state.settings.delete_files_after_compression;

                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            image_service.compress_image(
                                &input,
                                &output,
                                scale,
                                width,
                                height,
                                quality,
                                format,
                                delete_original,
                            )
                        })
                        .await
                        .unwrap()
                    },
                    Message::CompressionCompleted,
                )
            }
            Message::FormatSelected(f) => {
                self.state.format = f;

                Task::none()
            }
            Message::QualityChanged(q) => {
                self.state.quality = q;

                Task::none()
            }
            Message::WidthChanged(w) => {
                if w.is_empty() {
                    self.state.width = None;
                    return Task::none();
                } else {
                    let old_value = self.state.width;
                    self.state.width = match w.parse::<u32>() {
                        Ok(val) => Some(val),
                        Err(_) => old_value,
                    };
                }

                Task::none()
            }
            Message::HeightChanged(h) => {
                if h.is_empty() {
                    self.state.height = None;
                } else {
                    let old_value = self.state.height;
                    self.state.height = match h.parse::<u32>() {
                        Ok(val) => Some(val),
                        Err(_) => old_value,
                    };
                }

                Task::none()
            }
            Message::CompressionScaleChanged(s) => {
                self.state.scale = s;

                Task::none()
            }
            Message::CompressionCompleted(e) => {
                self.state.is_compressing = false;

                if let Err(err) = e {
                    error!("Compression failed: {err}");

                    self.state.status = format!("Error: {err}");
                    return Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(10)),
                        |_| Message::ClearStatus,
                    );
                };

                self.state.compression_succeeded = true;
                Task::none()
            }
            Message::IgnoreQuality(_e) => Task::none(),
            Message::IgnoreScale(_e) => Task::none(),
            Message::IgnoreFormatSelected(_e) => Task::none(),
            Message::OpenSettings => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::position(*last_window)
                    .then(|_| {
                        let icon_bytes = include_bytes!("../../resources/compressr.png");
                        let image = image::load_from_memory(icon_bytes).unwrap();
                        let window_icon =
                            window::icon::from_rgba(image.as_bytes().to_vec(), 256, 256)
                                .expect("Failed to load window icon");

                        let settings = window::Settings {
                            size: Size::new(450.0, 250.0),
                            resizable: true,
                            position: Position::Centered,
                            transparent: true,
                            decorations: true,
                            blur: true,
                            icon: Some(window_icon),
                            #[cfg(target_os = "linux")]
                            platform_specific: PlatformSpecific {
                                application_id: "com.codedead.compressr".to_string(),
                                ..PlatformSpecific::default()
                            },
                            ..window::Settings::default()
                        };

                        let (_, open) = window::open(settings);
                        open
                    })
                    .map(Message::SettingsViewOpened)
            }
            Message::AutoUpdateToggled(auto_update) => {
                self.state.settings.auto_update = auto_update;
                self.state.settings.save();

                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.windows
                    .values_mut()
                    .for_each(|window| window.theme = theme.clone());
                self.state.settings.theme = Some(theme.to_string());
                self.state.settings.save();
                Task::none()
            }
            Message::DeleteFilesAfterCompressionToggled(delete) => {
                self.state.settings.delete_files_after_compression = delete;
                self.state.settings.save();
                Task::none()
            }
            Message::ClearStatus => {
                self.state.status.clear();
                Task::none()
            }
            Message::ResetSettings => {
                self.state.settings = crate::components::settings::Settings::default();
                self.windows.values_mut().for_each(|window| {
                    window.theme = ThemeService::string_to_theme(
                        &self
                            .state
                            .settings
                            .theme
                            .clone()
                            .unwrap_or(Theme::Oxocarbon.to_string()),
                    )
                });

                Task::none()
            }
        }
    }

    /// Generates the view for a given window ID, returning an `Element` that represents the UI for that window.
    ///
    /// # Arguments
    ///
    /// * `window_id` - The ID of the window for which to generate the view.
    ///
    /// # Returns
    ///
    /// An `Element` representing the UI for the specified window. If the window ID is not found in the `windows` map, an empty space element is returned.
    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            match window.window_id {
                0 => main_view::view(&self.state),
                1 => settings_view::view(&self.state),
                _ => space().into(),
            }
        } else {
            space().into()
        }
    }

    /// Retrieves the theme associated with a specific window ID.
    ///
    /// # Arguments
    ///
    /// * `window` - The ID of the window for which to retrieve the theme.
    ///
    /// # Returns
    ///
    /// An `Option<Theme>` containing the theme of the specified window if it exists, or `None` if the window ID is not found in the `windows` map.
    pub fn theme(&self, window: window::Id) -> Option<Theme> {
        Some(self.windows.get(&window)?.theme.clone())
    }

    /// Retrieves the current scale factor for a specific window ID.
    ///
    /// # Arguments
    ///
    /// * `window` - The ID of the window for which to retrieve the scale factor.
    ///
    /// # Returns
    ///
    /// A `f32` representing the current scale factor of the specified window. If the window ID is not found in the `windows` map, a default value of `1.0` is returned.
    pub fn scale_factor(&self, window: window::Id) -> f32 {
        self.windows
            .get(&window)
            .map(|window| window.current_scale)
            .unwrap_or(1.0)
    }

    /// Subscribes to window close events, mapping them to `Message::WindowClosed` with the corresponding window ID.
    ///
    /// # Returns
    ///
    /// A `Subscription<Message>` that listens for window close events and produces a `Message::WindowClosed` with the ID of the closed window when such an event occurs.
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}

impl Window {
    /// Initializes a new `Window` instance with the specified title, window ID, and theme.
    ///
    /// # Arguments
    ///
    /// * `title` - A `String` representing the title of the window.
    /// * `window_id` - A `u8` representing the unique identifier for the window.
    /// * `theme` - A `Theme` representing the visual theme of the window
    ///
    /// # Returns
    ///
    /// A new instance of the `Window` struct initialized with the provided title, window ID, and theme, and a default scale factor of `1.0`.
    fn new(title: String, window_id: u8, theme: Theme) -> Self {
        Self {
            title,
            current_scale: 1.0,
            theme,
            window_id,
        }
    }
}
