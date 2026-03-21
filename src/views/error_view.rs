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
    let current_language = state
        .languages
        .iter()
        .find(|l| l.language_key == state.settings.language_key);
    let current_language = current_language.unwrap_or(&state.languages[0]);

    let mut has_error_to_copy = true;
    let last_error_message = match state.last_error_message.clone() {
        None => {
            has_error_to_copy = false;
            current_language.unknown_error.clone()
        }
        Some(e) => e,
    };

    let header = get_header(
        current_language.compressr_error.clone(),
        color!(175, 0, 0, 0.8),
    );

    let content = iced::widget::column![
        row![text(last_error_message)],
        row![space::vertical(),],
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
        .center_x(Length::Fill)
        .into()
}
