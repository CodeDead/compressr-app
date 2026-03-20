use crate::components::app::Message;
use crate::components::header::get_header;
use iced::widget::{button, container, row, space, text};
use iced::{Element, Length, color};

/// Builds the about view of the application, informing users about the application.
///
/// # Returns
///
/// An Element representing the about view of the application, which can be rendered by the Iced framework.
pub fn view() -> Element<'static, Message> {
    let header = get_header("Compressr - About".to_string(), color!(48, 48, 48, 0.8));

    let current_semver = env!("CARGO_PKG_VERSION").to_string();

    let content = iced::widget::column![
        row![text(format!(
            "Compressr was created with <3 by DeadLine.\n\nUI Framework: iced.rs\nVersion: {}\n\nCopyright © 2026 CodeDead",
            current_semver
        ))],
        row![space::vertical()],
        row![
            button("Website")
                    .width(Length::Shrink)
                    .on_press(Message::OpenCodeDeadPage),
            space::horizontal().width(Length::Fill),
            button("Donate")
                .width(Length::Shrink)
                .on_press(Message::OpenDonationPage),
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
