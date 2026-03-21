use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};

/// Builds the about view of the application, informing users about the application.
///
/// # Returns
///
/// An Element representing the about view of the application, which can be rendered by the Iced framework.
pub fn view(state: &State) -> Element<'_, Message> {
    let current_language = state
        .languages
        .iter()
        .find(|l| l.language_key == state.settings.language_key);
    let current_language = current_language.unwrap_or(&state.languages[0]);

    let current_semver = env!("CARGO_PKG_VERSION").to_string();

    let header = get_header(
        current_language.compressr_about.clone(),
        color!(48, 48, 48, 0.8),
    );

    let content = iced::widget::column![
        row![text(
            current_language
                .about_text
                .replace("{version}", &current_semver)
        )],
        row![space::vertical()],
        row![
            button(current_language.website.as_str())
                .width(Length::Shrink)
                .on_press(Message::OpenCodeDeadPage),
            space::horizontal().width(Length::Fill),
            button(current_language.donate.as_str())
                .width(Length::Shrink)
                .on_press(Message::OpenDonationPage),
        ]
    ]
    .spacing(15)
    .padding(15);

    let together = iced::widget::column![header, content];

    container(together)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
