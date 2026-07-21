use crate::components::app::Message;
use crate::components::header::{HEADER_BG_ERROR, get_header};
use crate::components::state::State;
use iced::widget::{button, container, row, scrollable, space, text};
use iced::{Element, Length};

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
    let current_language = state.current_language();

    let last_error_message = state
        .last_error_message
        .as_deref()
        .unwrap_or(&current_language.unknown_error)
        .to_string();
    let has_error_to_copy = state.last_error_message.is_some();

    let header = get_header(current_language.compressr_error.clone(), HEADER_BG_ERROR);

    // The message area is scrollable so long (multi-error) texts stay readable
    // while the action buttons remain pinned at the bottom.
    let content = iced::widget::column![
        scrollable(row![text(last_error_message)]).height(Length::Fill),
        row![
            has_error_to_copy.then(|| {
                button(current_language.copy.as_str())
                    .width(Length::Shrink)
                    .style(button::subtle)
                    .on_press(Message::CopyError)
            }),
            space::horizontal().width(Length::Fill),
            button(current_language.close.as_str())
                .width(Length::Shrink)
                .on_press(Message::CloseErrorView),
        ]
    ]
    .spacing(15)
    .padding(15);

    let together = iced::widget::column![header, content];

    container(together)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
