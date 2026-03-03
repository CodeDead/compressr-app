use crate::components::app::Message;
use crate::components::state::State;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};
use crate::components::header::get_header;

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
    let header = get_header("Compressr - Update".to_string(), color!(48, 48, 48, 0.8));

    let new_version = state
        .update_version
        .clone()
        .unwrap_or("unknown".to_string());
    let content = iced::widget::column![
        row![text(format!(
            "Version {} is now available! Would you like to download this version?",
            new_version
        ))],
        row![
            button("Information")
                .style(button::secondary)
                .width(Length::Shrink)
                .on_press(Message::OpenUpdateInformation),
            space::horizontal().width(Length::Fill),
            button("Download")
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
