use iced::{color, Color, Length};
use iced::widget::{container, row, space, text, Column};
use crate::components::app::Message;

/// Retrieve a header component for the application
///
/// # Arguments
///
/// * `title` - The title to be displayed in the header
/// * `background_color` - The background color of the header
///
/// # Returns
///
/// A Column widget representing the header, which can be used in the application's views to maintain a consistent header design across different screens.
pub fn get_header(title: String, background_color: Color) -> Column<'static, Message> {
    let header = iced::widget::column![row![
        container(iced::widget::column![row![
            text(title)
                .size(20)
                .width(Length::Shrink)
                .color(color!(255, 255, 255)),
            space::horizontal().width(Length::Fill),
        ]])
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(50)
        .padding(10)
        .style(move |_| container::Style {
            text_color: Default::default(),
            background: Some(iced::Background::Color(background_color)),
            border: Default::default(),
            shadow: iced::Shadow {
                color: color!(0, 0, 0, 0.2),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
            snap: false,
        })
    ]];

    header
}
