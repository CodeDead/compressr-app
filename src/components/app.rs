use crate::components::state::State;
use crate::services::image_service::{ImageService, OutputFormat};
use crate::views::{main_view, settings_view};
use iced::widget::space;
use iced::window::Position;
use iced::window::settings::PlatformSpecific;
use iced::{Element, Size, Subscription, Task, Theme, window};
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
}

pub struct App {
    pub(crate) windows: BTreeMap<window::Id, Window>,
    pub(crate) state: State,
    pub(crate) image_service: ImageService,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let icon_bytes = include_bytes!("../../resources/compressr.png");
        let image = image::load_from_memory(icon_bytes).unwrap();
        let window_icon = window::icon::from_rgba(image.as_bytes().to_vec(), 256, 256)
            .expect("Failed to load window icon");

        let settings = window::Settings {
            size: Size::new(650.0, 400.0),
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

    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainViewOpened(id) => {
                let window = Window::new("Compressr".to_string(), 0);

                self.windows.insert(id, window);
                Task::none()
            }
            Message::SettingsViewOpened(id) => {
                let window = Window::new("Compressr - Settings".to_string(), 1);

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
                if let Some(path) = FileDialog::new().save_file() {
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

                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            image_service.compress_image(
                                &input, &output, scale, width, height, quality, format,
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

                match e {
                    Ok(_) => self.state.compression_succeeded = true,
                    Err(err) => self.state.status = format!("Error: {err}"),
                };

                Task::none()
            }
            Message::IgnoreQuality(_e) => {
                /* No state change, used to prevent updates during slider dragging */
                Task::none()
            }
            Message::IgnoreScale(_e) => {
                /* No state change, used to prevent updates during slider dragging */
                Task::none()
            }
            Message::IgnoreFormatSelected(_e) => {
                /* No state change, used to prevent updates during pick list interaction */
                Task::none()
            }
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
                            size: Size::new(300.0, 400.0),
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
        }
    }

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

    pub fn theme(&self, window: window::Id) -> Option<Theme> {
        Some(self.windows.get(&window)?.theme.clone())
    }

    pub fn scale_factor(&self, window: window::Id) -> f32 {
        self.windows
            .get(&window)
            .map(|window| window.current_scale)
            .unwrap_or(1.0)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}

impl Window {
    fn new(title: String, window_id: u8) -> Self {
        Self {
            title,
            current_scale: 1.0,
            theme: Theme::Oxocarbon,
            window_id,
        }
    }
}
