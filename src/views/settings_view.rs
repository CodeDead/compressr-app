use crate::components::app::Message;
use crate::components::state::State;
use crate::services::theme_service::ThemeService;
use iced::widget::{button, checkbox, container, pick_list, row, space, text};
use iced::{Element, Length, Theme, color};

pub fn view(state: &State) -> Element<'_, Message> {
    let header = iced::widget::column![row![
        container(iced::widget::column![row![
            text("Compressr - Settings")
                .size(22)
                .width(Length::Shrink)
                .color(color!(255, 255, 255)),
            space::horizontal().width(Length::Fill),
        ]])
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(50)
        .padding(10)
        .style(|_| container::Style {
            text_color: Default::default(),
            background: Some(iced::Background::Color(color!(48, 48, 48, 0.8))),
            border: Default::default(),
            shadow: iced::Shadow {
                color: color!(0, 0, 0, 0.2),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
            snap: false,
        })
    ]];

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
        row![
            space::horizontal().width(Length::Fill),
            button("Reset to defaults")
                .style(button::danger)
                .width(Length::Shrink)
                .on_press(Message::ResetSettings),
        ]
    ]
    .spacing(15)
    .padding(15);

    let together = iced::widget::column![header, content].spacing(10);

    container(together)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
