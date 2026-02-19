use crate::services::image_service::{ImageService, OutputFormat};
use iced::widget::{Image as iced_image, Text, image};
use iced::widget::{button, container, pick_list, row, slider, space, text, text_input};
use iced::{Element, Length, Task, color};
use iced_aw::{helpers::badge, style};
use rfd::FileDialog;

struct State {
    input_path: String,
    output_path: String,
    scale: u32,
    height: Option<u32>,
    width: Option<u32>,
    quality: u8,
    format: OutputFormat,
    is_compressing: bool,
    compression_succeeded: bool,
    status: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            input_path: String::new(),
            output_path: String::new(),
            scale: 100,
            height: None,
            width: None,
            quality: 100,
            format: OutputFormat::Jpeg,
            is_compressing: false,
            compression_succeeded: false,
            status: String::new(),
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
    CompressionCompleted(Result<(), String>),
    FormatSelected(OutputFormat),
    QualityChanged(u8),
    WidthChanged(String),
    HeightChanged(String),
    ScaleChanged(u32),
    OpenSettings,
    IgnoreQuality(u8),
    IgnoreScale(u32),
    IgnoreFormatSelected(OutputFormat),
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
                self.state.compression_succeeded = false;
            }
            Message::SelectOutput => {
                if let Some(path) = FileDialog::new().save_file() {
                    self.state.output_path = path.display().to_string();
                }
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

                return Task::perform(
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
                );
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
            Message::OpenSettings => todo!(),
            Message::CompressionCompleted(e) => {
                self.state.is_compressing = false;

                match e {
                    Ok(_) => self.state.compression_succeeded = true,
                    Err(err) => self.state.status = format!("Error: {err}"),
                }
            }
            Message::IgnoreQuality(_e) => { /* No state change, used to prevent updates during slider dragging */
            }
            Message::IgnoreScale(_e) => { /* No state change, used to prevent updates during slider dragging */
            }
            Message::IgnoreFormatSelected(_e) => { /* No state change, used to prevent updates during pick list selection */
            }
        }

        Task::none()
    }

    /// Builds the view based on the current state and returns it as an `Element<Message>`.
    ///
    /// # Returns
    ///
    /// An `Element<Message>` that represents the current view of the application.
    pub fn view(&self) -> Element<'_, Message> {
        let bytes = include_bytes!("../../resources/settings.png");
        let handle = image::Handle::from_bytes(bytes.as_slice());
        let image = iced_image::new(handle);

        let mut browse_input_button = button("Browse");
        let mut browse_output_button = button("Browse");

        let mut quality_slider = slider(0..=100, self.state.quality, Message::IgnoreQuality);
        let mut scale_slider = slider(0..=100, self.state.scale, Message::IgnoreScale);

        let mut format_pick_list = pick_list(
            &OutputFormat::ALL[..],
            Some(self.state.format),
            Message::IgnoreFormatSelected,
        );

        let mut compress_button = button("Compress");

        if !self.state.is_compressing {
            browse_input_button = browse_input_button.on_press(Message::SelectInput);
            browse_output_button = browse_output_button.on_press(Message::SelectOutput);

            quality_slider = slider(0..=100, self.state.quality, Message::QualityChanged);
            scale_slider = slider(0..=100, self.state.scale, Message::ScaleChanged);

            format_pick_list = pick_list(
                &OutputFormat::ALL[..],
                Some(self.state.format),
                Message::FormatSelected,
            );

            compress_button = compress_button.on_press(Message::Compress);
        }

        let header = iced::widget::column![row![
            container(iced::widget::column![row![
                text("Compressr")
                    .size(22)
                    .width(Length::Shrink)
                    .color(color!(255, 255, 255)),
                space::horizontal().width(Length::Fill),
                button(image.width(28).height(28))
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .on_press(Message::OpenSettings)
            ]])
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(50)
            .padding(10)
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
                self.state
                    .status
                    .starts_with("Error")
                    .then(|| badge(Text::new(&self.state.status)).style(style::badge::danger)),
            ],
            row![
                container(text("Input:")).width(Length::FillPortion(1)),
                container(text_input("", &self.state.input_path).width(Length::Fill))
                    .width(Length::FillPortion(3)),
                container(browse_input_button).width(Length::Shrink),
            ],
            row![
                container(text("Output:")).width(Length::FillPortion(1)),
                container(text_input("", &self.state.output_path).width(Length::Fill))
                    .width(Length::FillPortion(3)),
                container(browse_output_button).width(Length::Shrink),
            ],
            row![text("Format: "), space::horizontal(), format_pick_list,],
            row![
                container(text("Quality:")).width(Length::FillPortion(1)),
                container(quality_slider).width(Length::FillPortion(3)),
                container(text(self.state.quality.to_string() + "%")).width(Length::Shrink),
            ]
            .spacing(10),
            row![
                container(text("Scale: ")).width(Length::FillPortion(1)),
                container(scale_slider).width(Length::FillPortion(3)),
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
                self.state
                    .is_compressing
                    .then(|| badge(Text::new("Compressing")).style(style::badge::info)),
                self.state
                    .compression_succeeded
                    .then(|| badge(Text::new("Compressed!")).style(style::badge::success)),
                space::horizontal(),
                compress_button,
            ],
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
