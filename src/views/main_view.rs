pub(crate) use crate::components::app::Message;
use crate::components::state::State;
use crate::services::image_service::OutputFormat;
use iced::widget::{Image, Text};
use iced::widget::{button, container, pick_list, row, slider, space, text, text_input};
use iced::{Element, Length, color};
use iced_aw::widget::LabeledFrame;
use iced_aw::{helpers::badge, number_input, style};

impl std::fmt::Display for OutputFormat {
    /// Formats the OutputFormat enum as a user-friendly string for display in the UI.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the string representation to.
    ///
    /// # Returns
    ///
    /// A Result indicating whether the formatting was successful.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Jpeg => write!(f, "JPEG"),
            OutputFormat::Png => write!(f, "PNG"),
            OutputFormat::Gif => write!(f, "GIF"),
            OutputFormat::WebP => write!(f, "WebP"),
            OutputFormat::Bmp => write!(f, "BMP"),
            OutputFormat::Tiff => write!(f, "Tiff"),
        }
    }
}

/// Builds the main view of the application, displaying the current state and providing controls for user interaction.
///
/// # Arguments
///
/// * `state` - A reference to the current application state, which contains information about the input/output paths, compression settings, and status.
///
/// # Returns
///
/// An Element representing the main view of the application, which can be rendered by the Iced framework.
pub fn view(state: &State) -> Element<'_, Message> {
    let mut dark_icons = false;
    if let Some(theme) = &state.settings.theme {
        dark_icons = theme == "Light"
            || theme == "Solarized Light"
            || theme == "Gruvbox Light"
            || theme == "Catppuccin Latte"
            || theme == "Tokyo Night Light"
            || theme == "Kanagawa Lotus";
    }

    let current_language = state
        .languages
        .iter()
        .find(|l| l.language_key == state.settings.language_key);
    let current_language = current_language.unwrap_or(&state.languages[0]);

    let mut text_input_path = text_input("", &state.input_path.join(", "));
    let mut text_output_path = text_input("", &state.output_path);

    let mut browse_input_button = button(current_language.browse.as_str());
    let mut browse_output_button = button(current_language.browse.as_str());

    let mut quality_slider = if state.format == OutputFormat::Jpeg {
        slider(1..=100, state.quality, Message::IgnoreQuality)
    } else {
        slider(1..=100, 100, Message::IgnoreQuality)
    };
    let mut scale_slider = slider(1..=100, state.scale, Message::IgnoreScale);

    let mut format_pick_list = pick_list(
        &OutputFormat::ALL[..],
        Some(state.format),
        Message::IgnoreFormatSelected,
    );

    let mut compress_button = button(current_language.compress.as_str());

    if !state.is_compressing {
        text_input_path = text_input_path.on_input(|_| Message::SelectInput);
        browse_input_button = browse_input_button.on_press(Message::SelectInput);

        text_output_path = text_output_path.on_input(|_| Message::SelectOutput);
        browse_output_button = browse_output_button.on_press(Message::SelectOutput);

        if state.format == OutputFormat::Jpeg {
            quality_slider = slider(1..=100, state.quality, Message::QualityChanged);
        }

        scale_slider = slider(1..=100, state.scale, Message::CompressionScaleChanged);

        format_pick_list = pick_list(
            &OutputFormat::ALL[..],
            Some(state.format),
            Message::FormatSelected,
        );

        compress_button = compress_button.on_press(Message::Compress);
    }

    let settings_image: Image = if dark_icons {
        Image::new(state.main_view_icons.settings_dark.clone())
    } else {
        Image::new(state.main_view_icons.settings.clone())
    };
    let info_image: Image = if dark_icons {
        Image::new(state.main_view_icons.info_dark.clone())
    } else {
        Image::new(state.main_view_icons.info.clone())
    };

    let header = iced::widget::column![row![
        container(iced::widget::column![row![
            text("Compressr")
                .size(20)
                .width(Length::Shrink)
                .color(color!(255, 255, 255)),
            space::horizontal().width(Length::Fill),
            button(settings_image.width(28).height(28))
                .style(button::subtle)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .on_press(Message::OpenSettings),
            space::horizontal().width(Length::Fixed(8.0)),
            button(info_image.width(28).height(28))
                .style(button::subtle)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .on_press(Message::OpenAbout),
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

    let width = state.width.unwrap_or(0) as i32;
    let height = state.height.unwrap_or(0) as i32;

    let content = iced::widget::column![
        row![
            container(text(current_language.input.as_str())).width(Length::FillPortion(1)),
            container(text_input_path).width(Length::FillPortion(3)),
            container(browse_input_button).width(Length::Shrink),
        ],
        row![
            container(text(current_language.output.as_str())).width(Length::FillPortion(1)),
            container(text_output_path).width(Length::FillPortion(3)),
            container(browse_output_button).width(Length::Shrink),
        ],
        row![
            text(current_language.format.as_str()),
            space::horizontal(),
            format_pick_list,
        ],
        row![
            container(text(current_language.quality.as_str())).width(Length::FillPortion(1)),
            container(quality_slider).width(Length::FillPortion(3)),
            container(text(state.quality.to_string() + "%")).width(Length::Shrink),
        ]
        .spacing(10),
        row![
            container(text(current_language.scale.as_str())).width(Length::FillPortion(1)),
            container(scale_slider).width(Length::FillPortion(3)),
            container(text(state.scale.to_string() + "%")).width(Length::Shrink),
        ]
        .spacing(10),
        row![
            LabeledFrame::new(
                current_language.width.as_str(),
                number_input(&width, 0..=i32::MAX, Message::WidthChanged)
                    .width(Length::Fill)
                    .step(1)
            )
            .width(Length::Fill),
            LabeledFrame::new(
                current_language.height.as_str(),
                number_input(&height, 0..=i32::MAX, Message::HeightChanged)
                    .width(Length::Fill)
                    .step(1),
            )
            .width(Length::Fill)
        ],
        row![space::vertical()],
        row![
            state
                .is_compressing
                .then(|| badge(Text::new(current_language.compressing.as_str()))
                    .style(style::badge::info)),
            state
                .compression_succeeded
                .then(|| badge(Text::new(current_language.compressed.as_str()))
                    .style(style::badge::success)),
            space::horizontal(),
            compress_button,
        ],
    ]
    .spacing(15)
    .padding(15);

    let together = iced::widget::column![header, content];

    container(together)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
