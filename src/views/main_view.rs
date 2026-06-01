pub(crate) use crate::components::app::Message;
use crate::components::state::State;
use crate::services::image_service::OutputFormat;
use iced::widget::{Image, progress_bar};
use iced::widget::{button, column, container, pick_list, row, slider, space, text, text_input};
use iced::{Element, Length, Theme, color};
use iced_aw::{DropDown, drop_down, number_input};

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
    const LABEL_WIDTH: f32 = 120.0;
    const DIM_LABEL_WIDTH: f32 = 80.0;

    let dark_icons = matches!(
        state.settings.theme,
        Theme::Light
            | Theme::SolarizedLight
            | Theme::GruvboxLight
            | Theme::CatppuccinLatte
            | Theme::TokyoNightLight
            | Theme::KanagawaLotus
    );

    let current_language = state.current_language();

    let mut text_input_path = text_input("", &state.input_path.join(", "));
    let mut text_output_path = text_input("", &state.output_path);

    let mut dropdown_trigger =
        button(row![text(current_language.browse.as_str()), text(" \u{25BE}"),].spacing(2));
    let mut browse_output_button = button(current_language.browse.as_str());

    let mut quality_slider =
        if state.format == OutputFormat::Jpeg || state.format == OutputFormat::WebP {
            slider(1..=100, state.quality, |_| Message::IgnoreQuality)
        } else {
            slider(1..=100, 100, |_| Message::IgnoreQuality)
        };
    let mut scale_slider = slider(1..=100, state.scale, |_| Message::IgnoreScale);

    let mut format_pick_list = pick_list(&OutputFormat::ALL[..], Some(state.format), |_| {
        Message::IgnoreFormatSelected
    });

    let mut compress_button = button(current_language.compress.as_str());

    if !state.is_compressing {
        text_input_path = text_input_path.on_input(|_| Message::ToggleInputDropdown);
        dropdown_trigger = dropdown_trigger.on_press(Message::ToggleInputDropdown);

        text_output_path = text_output_path.on_input(|_| Message::SelectOutput);
        browse_output_button = browse_output_button.on_press(Message::SelectOutput);

        if state.format == OutputFormat::Jpeg || state.format == OutputFormat::WebP {
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

    let dropdown_overlay = container(
        column![
            button(text(current_language.select_files.as_str()))
                .on_press(Message::SelectInput)
                .width(Length::Fill),
            button(text(current_language.select_folder.as_str()))
                .on_press(Message::SelectInputFolder)
                .width(Length::Fill),
        ]
        .spacing(4)
        .padding(4),
    )
    .width(Length::Fixed(160.0));

    let browse_input_dropdown = DropDown::new(
        dropdown_trigger,
        dropdown_overlay,
        state.show_input_dropdown,
    )
    .on_dismiss(Message::DismissInputDropdown)
    .alignment(drop_down::Alignment::Bottom);

    let width = state.width.unwrap_or(0) as i32;
    let height = state.height.unwrap_or(0) as i32;

    let width_input = if state.is_compressing {
        number_input(&width, 0..=i32::MAX, |_| Message::IgnoreWidth)
            .width(Length::Fill)
            .step(1)
    } else {
        number_input(&width, 0..=i32::MAX, Message::WidthChanged)
            .width(Length::Fill)
            .step(1)
    };

    let height_input = if state.is_compressing {
        number_input(&height, 0..=i32::MAX, |_| Message::IgnoreHeight)
            .width(Length::Fill)
            .step(1)
    } else {
        number_input(&height, 0..=i32::MAX, Message::HeightChanged)
            .width(Length::Fill)
            .step(1)
    };

    let content = iced::widget::column![
        row![
            text(current_language.input.as_str()).width(Length::Fixed(LABEL_WIDTH)),
            text_input_path.width(Length::Fill),
            container(browse_input_dropdown).width(Length::Shrink),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            text(current_language.output.as_str()).width(Length::Fixed(LABEL_WIDTH)),
            text_output_path.width(Length::Fill),
            browse_output_button.width(Length::Shrink),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            text(current_language.format.as_str()).width(Length::Fixed(LABEL_WIDTH)),
            format_pick_list.width(Length::Fill),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            text(current_language.quality.as_str()).width(Length::Fixed(LABEL_WIDTH)),
            quality_slider.width(Length::Fill),
            text(state.quality.to_string() + "%").width(Length::Shrink),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            text(current_language.scale.as_str()).width(Length::Fixed(LABEL_WIDTH)),
            scale_slider.width(Length::Fill),
            text(state.scale.to_string() + "%").width(Length::Shrink),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            row![
                text(current_language.width.as_str()).width(Length::Fixed(DIM_LABEL_WIDTH)),
                width_input.width(Length::Fill),
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center)
            .width(Length::Fill),
            row![
                text(current_language.height.as_str()).width(Length::Fixed(DIM_LABEL_WIDTH)),
                height_input.width(Length::Fill),
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center)
            .width(Length::Fill),
        ]
        .spacing(15),
        space::vertical(),
        {
            let status_widget: Element<'_, Message> = if state.is_compressing {
                let progress = if state.progress_total > 0 {
                    state.progress_completed as f32 / state.progress_total as f32
                } else {
                    0.0
                };
                row![
                    progress_bar(0.0..=1.0, progress),
                    text(
                        current_language
                            .compressing_progress
                            .replace("{completed}", &state.progress_completed.to_string())
                            .replace("{total}", &state.progress_total.to_string())
                    ),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center)
                .into()
            } else {
                row![space::horizontal(), compress_button,].into()
            };
            row![status_widget]
        },
    ]
    .spacing(15)
    .padding(15)
    .height(Length::Fill);

    let together = iced::widget::column![header, content].height(Length::Fill);

    container(together)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
