use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use crate::services::theme_service::ThemeService;
use iced::widget::{button, checkbox, container, pick_list, row, space, text};
use iced::{Element, Length, Theme, color};

/// Builds the settings view of the application, allowing users to adjust preferences such as auto-update, file deletion after compression, and theme selection.
///
/// # Arguments
///
/// * `state` - A reference to the current application state, which contains the user's settings and preferences.
///
/// # Returns
///
/// An Element representing the settings view of the application, which can be rendered by the Iced framework.
pub fn view(state: &State) -> Element<'_, Message> {
    let header = get_header("Compressr - Settings".to_string(), color!(48, 48, 48, 0.8));

    let content = iced::widget::column![
        row![
            checkbox(state.settings.auto_update)
                .label("Automatically check for updates")
                .on_toggle(Message::AutoUpdateToggled)
        ],
        row![
            checkbox(state.settings.delete_files_after_compression)
                .label("Delete original files after compression")
                .on_toggle(Message::DeleteFilesAfterCompressionToggled)
        ],
        row![
            text("Theme:").width(Length::Shrink),
            pick_list(
                Theme::ALL,
                Some(ThemeService::string_to_theme(
                    &state
                        .settings
                        .theme
                        .clone()
                        .unwrap_or(Theme::Oxocarbon.to_string())
                )),
                Message::ThemeChanged
            )
            .placeholder("Choose a theme...")
            .width(Length::Fill)
        ]
        .spacing(20),
        row![space::vertical(),],
        row![
            button("Check for updates")
                .width(Length::Shrink)
                .on_press(Message::CheckForUpdates),
            space::horizontal().width(Length::Fill),
            button("Reset all settings")
                .style(button::danger)
                .width(Length::Shrink)
                .on_press(Message::ResetSettings),
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
