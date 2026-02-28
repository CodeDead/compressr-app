pub(crate) use crate::components::app::Message;
use crate::components::state::State;
use crate::services::image_service::OutputFormat;
use iced::widget::{Image as iced_image, Text, image as image_widget};
use iced::widget::{button, container, pick_list, row, slider, space, text, text_input};
use iced::{Element, Length, color};
use iced_aw::{helpers::badge, style};

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
pub fn view(state: &State) -> Element<'_, Message> {
    let bytes = include_bytes!("../../resources/settings.png");
    let handle = image_widget::Handle::from_bytes(bytes.as_slice());
    let image = iced_image::new(handle);

    let mut browse_input_button = button("Browse");
    let mut browse_output_button = button("Browse");

    let mut quality_slider = slider(0..=100, state.quality, Message::IgnoreQuality);
    let mut scale_slider = slider(0..=100, state.scale, Message::IgnoreScale);

    let mut format_pick_list = pick_list(
        &OutputFormat::ALL[..],
        Some(state.format),
        Message::IgnoreFormatSelected,
    );

    let mut compress_button = button("Compress");

    if !state.is_compressing {
        browse_input_button = browse_input_button.on_press(Message::SelectInput);
        browse_output_button = browse_output_button.on_press(Message::SelectOutput);

        quality_slider = slider(0..=100, state.quality, Message::QualityChanged);
        scale_slider = slider(0..=100, state.scale, Message::CompressionScaleChanged);

        format_pick_list = pick_list(
            &OutputFormat::ALL[..],
            Some(state.format),
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
                .on_press(Message::OpenSettings),
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
            state
                .status
                .starts_with("Error")
                .then(|| badge(Text::new(&state.status)).style(style::badge::danger)),
        ],
        row![
            container(text("Input:")).width(Length::FillPortion(1)),
            container(text_input("", &state.input_path).width(Length::Fill))
                .width(Length::FillPortion(3)),
            container(browse_input_button).width(Length::Shrink),
        ],
        row![
            container(text("Output:")).width(Length::FillPortion(1)),
            container(text_input("", &state.output_path).width(Length::Fill))
                .width(Length::FillPortion(3)),
            container(browse_output_button).width(Length::Shrink),
        ],
        row![text("Format: "), space::horizontal(), format_pick_list,],
        row![
            container(text("Quality:")).width(Length::FillPortion(1)),
            container(quality_slider).width(Length::FillPortion(3)),
            container(text(state.quality.to_string() + "%")).width(Length::Shrink),
        ]
        .spacing(10),
        row![
            container(text("Scale: ")).width(Length::FillPortion(1)),
            container(scale_slider).width(Length::FillPortion(3)),
            container(text(state.scale.to_string() + "%")).width(Length::Shrink),
        ]
        .spacing(10),
        row![
            text_input(
                "Width",
                &match state.width {
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
                &match state.height {
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
            state
                .is_compressing
                .then(|| badge(Text::new("Compressing")).style(style::badge::info)),
            state
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
