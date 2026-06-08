pub(crate) use crate::components::app::Message;
use crate::components::header::get_header_with_actions;
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

    // Controls are interactive only while idle. iced disables widgets whose
    // handlers are omitted, so we attach `on_*` callbacks conditionally rather
    // than constructing separate enabled/disabled variants.
    let enabled = !state.is_compressing;
    let quality_enabled =
        enabled && (state.format == OutputFormat::Jpeg || state.format == OutputFormat::WebP);

    let text_input_path = text_input("", &state.input_path.join(", "))
        .on_input_maybe(enabled.then_some(|_| Message::ToggleInputDropdown));
    let text_output_path = text_input("", &state.output_path)
        .on_input_maybe(enabled.then_some(|_| Message::SelectOutput));

    let dropdown_trigger =
        button(row![text(current_language.browse.as_str()), text(" \u{25BE}"),].spacing(2))
            .on_press_maybe(enabled.then_some(Message::ToggleInputDropdown));
    let browse_output_button = button(current_language.browse.as_str())
        .on_press_maybe(enabled.then_some(Message::SelectOutput));

    // Sliders require an on-change handler at construction, so the disabled
    // variant maps to a no-op while the displayed value stays meaningful.
    let quality_slider = if quality_enabled {
        slider(1..=100, state.quality, Message::QualityChanged)
    } else {
        slider(1..=100, state.quality, |_| Message::Noop)
    };
    let scale_slider = if enabled {
        slider(1..=100, state.scale, Message::CompressionScaleChanged)
    } else {
        slider(1..=100, state.scale, |_| Message::Noop)
    };

    let format_pick_list = if enabled {
        pick_list(
            &OutputFormat::ALL[..],
            Some(state.format),
            Message::FormatSelected,
        )
    } else {
        pick_list(&OutputFormat::ALL[..], Some(state.format), |_| {
            Message::Noop
        })
    };

    let compress_button = button(current_language.compress.as_str())
        .on_press_maybe(enabled.then_some(Message::Compress));

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

    let header = get_header_with_actions(
        "Compressr".to_string(),
        color!(48, 48, 48, 0.8),
        vec![
            button(settings_image.width(28).height(28))
                .style(button::subtle)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .on_press(Message::OpenSettings)
                .into(),
            space::horizontal().width(Length::Fixed(8.0)).into(),
            button(info_image.width(28).height(28))
                .style(button::subtle)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .on_press(Message::OpenAbout)
                .into(),
        ],
    );

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

    // `number_input` requires an on-change handler at construction (for its
    // spinner buttons), so the disabled variant maps to a no-op like the sliders.
    let width_input = if enabled {
        number_input(&width, 0..=i32::MAX, Message::WidthChanged)
    } else {
        number_input(&width, 0..=i32::MAX, |_| Message::Noop)
    }
    .width(Length::Fill)
    .step(1);

    let height_input = if enabled {
        number_input(&height, 0..=i32::MAX, Message::HeightChanged)
    } else {
        number_input(&height, 0..=i32::MAX, |_| Message::Noop)
    }
    .width(Length::Fill)
    .step(1);

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
            text(format!("{}%", state.quality)).width(Length::Shrink),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            text(current_language.scale.as_str()).width(Length::Fixed(LABEL_WIDTH)),
            scale_slider.width(Length::Fill),
            text(format!("{}%", state.scale)).width(Length::Shrink),
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
