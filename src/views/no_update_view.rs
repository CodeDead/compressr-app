use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};

/// Builds the no-update view of the application, informing users that they already have the latest version installed.
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
