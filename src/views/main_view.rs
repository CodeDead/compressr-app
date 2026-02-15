use crate::services::image_service::{ImageService, OutputFormat};
use iced::widget::{button, container, pick_list, row, slider, space, text, text_input};
use iced::{Element, Length, Task, color};
use rfd::FileDialog;

struct State {
    status: String,
    input_path: String,
    output_path: String,
    scale: u32,
    height: Option<u32>,
    width: Option<u32>,
    quality: u8,
    format: OutputFormat,
}

impl Default for State {
    fn default() -> Self {
        State {
            status: String::new(),
            input_path: String::new(),
            output_path: String::new(),
            scale: 100,
            height: None,
            width: None,
            quality: 100,
            format: OutputFormat::Jpeg,
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Jpeg => write!(f, "JPEG"),
            OutputFormat::Png => write!(f, "PNG"),
            OutputFormat::Gif => write!(f, "GIF"),
            OutputFormat::WebP => write!(f, "WebP"),
        }
    }
}
#[derive(Default)]
pub struct MainView {
    state: State,
    image_service: ImageService,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectInput,
    SelectOutput,
    Compress,
    FormatSelected(OutputFormat),
    QualityChanged(u8),
    WidthChanged(String),
    HeightChanged(String),
    ScaleChanged(u32),
}

impl MainView {
    /// Updates the state based on the given message and returns any necessary tasks.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to process.
    ///
    /// # Returns
    ///
    /// A `Task<Message>` that represents any asynchronous work that needs to be done as a result of processing the message.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectInput => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.state.input_path = path.display().to_string();
                }
            }
            Message::SelectOutput => {
                if let Some(path) = FileDialog::new().save_file() {
                    self.state.output_path = path.display().to_string();
                }
            }
            Message::Compress => {
                self.state.status = "⏳ Compressing...".into();
                match self.image_service.compress_image(
                    &self.state.input_path,
                    &self.state.output_path,
                    self.state.scale,
                    self.state.width,
                    self.state.height,
                    self.state.quality,
                    self.state.format,
                ) {
                    Ok(_) => self.state.status = "✅ Compression successful".into(),
                    Err(e) => self.state.status = format!("❌ Error: {}", e),
                }
            }
            Message::FormatSelected(f) => self.state.format = f,
            Message::QualityChanged(q) => self.state.quality = q,
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
            }
            Message::ScaleChanged(s) => self.state.scale = s,
        }

        Task::none()
    }

    /// Builds the view based on the current state and returns it as an `Element<Message>`.
    ///
    /// # Returns
    ///
    /// An `Element<Message>` that represents the current view of the application.
    pub fn view(&self) -> Element<'_, Message> {
        let header = iced::widget::column![row![
            container(
                text(" Compressr")
                    .size(25)
                    .width(Length::Shrink)
                    .color(color!(255, 255, 255))
            )
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(50)
            .style(|_| container::Style {
                text_color: Default::default(),
                background: Some(iced::Background::Color(color!(48, 48, 48, 0.8))),
                border: Default::default(),
                shadow: iced::Shadow {
                    color: color!(0, 0, 0, 0.2),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 5.0,
                },
                snap: false,
            })
        ]];
        let content = iced::widget::column![
            row![
                text("Input: "),
                text(self.state.input_path.clone()).width(Length::Fill),
                button("Browse").on_press(Message::SelectInput)
            ],
            row![
                text("Output: "),
                text(self.state.output_path.clone()).width(Length::Fill),
                button("Browse").on_press(Message::SelectOutput)
            ],
            row![
                text("Format: "),
                space::horizontal(),
                pick_list(
                    &OutputFormat::ALL[..],
                    Some(self.state.format),
                    Message::FormatSelected
                )
            ],
            row![
                container(text("Quality:")).width(Length::FillPortion(1)),
                container(slider(0..=100, self.state.quality, Message::QualityChanged))
                    .width(Length::FillPortion(3)),
                container(text(self.state.quality.to_string() + "%")).width(Length::Shrink),
            ]
            .spacing(10),
            row![
                container(text("Scale: ")).width(Length::FillPortion(1)),
                container(slider(0..=100, self.state.scale, Message::ScaleChanged))
                    .width(Length::FillPortion(3)),
                container(text(self.state.scale.to_string() + "%")).width(Length::Shrink),
            ]
            .spacing(10),
            row![
                text_input(
                    "Width",
                    &match self.state.width {
                        None => {
                            String::new()
                        }
                        Some(v) => {
                            v.to_string()
                        }
                    }
                )
                .on_input(Message::WidthChanged),
                text_input(
                    "Height",
                    &match self.state.height {
                        None => {
                            String::new()
                        }
                        Some(v) => {
                            v.to_string()
                        }
                    }
                )
                .on_input(Message::HeightChanged),
            ],
            row![
                space::horizontal(),
                button("Compress").on_press(Message::Compress),
            ],
            text(&self.state.status)
        ]
        .spacing(15)
        .padding(15);

        let together = iced::widget::column![header, content].spacing(10);

        container(together)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }
}
