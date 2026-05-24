use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};

/// Builds the no-update view of the application, informing users that they already have the latest version installed.
///
/// # Arguments
///
/// * `state` - A reference to the current application state, which contains information about the current language and other relevant data.
///
/// # Returns
///
/// An Element representing the no-update view of the application, which can be rendered by the Iced framework.
pub fn view(state: &State) -> Element<'_, Message> {
    let current_language = state.current_language();

    let header = get_header(
        current_language.compressr_update.clone(),
        color!(48, 48, 48, 0.8),
    );

    let content = iced::widget::column![
        row![text(current_language.latest_version_installed.as_str())],
        row![space::vertical()],
        row![
            space::horizontal().width(Length::Fill),
            button(current_language.close.as_str())
                .width(Length::Shrink)
                .on_press(Message::CloseNoUpdateView),
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
