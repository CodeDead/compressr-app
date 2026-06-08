use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
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
    let current_language = state.current_language();

    let header = get_header(
        current_language.compressr_settings.clone(),
        color!(48, 48, 48, 0.8),
    );

    let content = iced::widget::column![
        row![
            checkbox(state.settings.auto_update)
                .label(current_language.automatically_check_for_updates.as_str())
                .on_toggle(Message::AutoUpdateToggled)
        ],
        row![
            checkbox(state.settings.show_compression_results)
                .label(current_language.show_compression_results.as_str())
                .on_toggle(Message::ShowCompressionResultsToggled)
        ],
        row![
            checkbox(state.settings.delete_files_after_compression)
                .label(
                    current_language
                        .delete_original_files_after_compression
                        .as_str()
                )
                .on_toggle(Message::DeleteFilesAfterCompressionToggled)
        ],
        row![
            checkbox(state.settings.preserve_exif)
                .label(current_language.preserve_exif_data.as_str())
                .on_toggle(Message::PreserveExifToggled)
        ],
        row![
            checkbox(state.settings.recursive_folder_scan)
                .label(current_language.recursive_folder_scan.as_str())
                .on_toggle(Message::RecursiveFolderScanToggled)
        ],
        row![
            text(current_language.theme.as_str()).width(Length::FillPortion(1)),
            pick_list(
                Theme::ALL,
                Some(state.settings.theme.clone()),
                Message::ThemeChanged
            )
            .placeholder(current_language.select_theme.as_str())
            .width(Length::FillPortion(3))
        ]
        .spacing(20),
        row![
            text(current_language.language.as_str()).width(Length::FillPortion(1)),
            pick_list(
                state
                    .languages
                    .iter()
                    .map(|l| l.language_name.clone())
                    .collect::<Vec<String>>(),
                Some(current_language.language_name.clone()),
                Message::LanguageChanged
            )
            .placeholder(current_language.select_language.as_str())
            .width(Length::FillPortion(3))
        ]
        .spacing(20),
        row![space::vertical(),],
        row![
            button(current_language.check_for_updates.as_str())
                .width(Length::Shrink)
                .on_press(Message::CheckForUpdates(true)),
            space::horizontal().width(Length::Fill),
            button(current_language.reset_all_settings.as_str())
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
