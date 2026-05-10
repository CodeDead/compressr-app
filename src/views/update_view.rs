use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};

/// Builds the update view of the application, informing users about available updates and providing options to download or learn more about the new version.
///
/// # Arguments
///
/// * `state` - A reference to the current application state, which contains information about available updates, including the new version number and URLs for downloading or learning more about the update.
///
/// # Returns
///
/// An Element representing the update view of the application, which can be rendered by the Iced framework.
pub fn view(state: &State) -> Element<'_, Message> {
    let current_language = state
        .languages
        .iter()
        .find(|l| l.language_key == state.settings.language_key);
    let current_language = current_language.unwrap_or(&state.languages[0]);

    let header = get_header(
        current_language.compressr_update.clone(),
        color!(48, 48, 48, 0.8),
    );

    let new_version = state.update_version.clone().unwrap_or("".to_string());

    let content = iced::widget::column![
        row![text(
            current_language
                .new_version_available
                .replace("{version}", &new_version)
        )],
        row![space::vertical()],
        row![
            state
                .update_info_url
                .is_some()
                .then(|| button(current_language.information.as_str())
                    .style(button::secondary)
                    .width(Length::Shrink)
                    .on_press(Message::OpenUpdateInformation)),
            space::horizontal().width(Length::Fill),
            button(current_language.close.as_str())
                .width(Length::Shrink)
                .on_press(Message::CloseUpdateView),
            text(" "),
            button(current_language.download.as_str())
                .style(button::primary)
                .width(Length::Shrink)
                .on_press(Message::DownloadUpdate),
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
