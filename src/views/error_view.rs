use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};

/// Builds the error view of the application, informing users about error messages
///
/// # Arguments
///
/// * `state` - A reference to the current application state, which contains information about the error message to be displayed.
///
/// # Returns
///
/// An Element representing the error view of the application, which can be rendered by the Iced framework.
pub fn view(state: &State) -> Element<'_, Message> {
    let header = get_header("Compressr - Error".to_string(), color!(175, 0, 0, 0.8));

    let last_error_message = state
        .last_error_message
        .clone()
        .unwrap_or("An unknown error occurred.".to_string());

    let content = iced::widget::column![
        row![text(format!("An error occurred:\n{}", last_error_message))],
        row![space::vertical(),],
        row![
            button("Copy")
                .width(Length::Shrink)
                .style(button::subtle)
                .on_press(Message::CopyError),
            space::horizontal().width(Length::Fill),
            button("Close")
                .width(Length::Shrink)
                .on_press(Message::CloseErrorView),
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
