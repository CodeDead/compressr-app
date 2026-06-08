use crate::components::app::Message;
use iced::widget::{Column, Row, container, row, space, text};
use iced::{Color, Element, Length, color};

/// Retrieve a header component for the application.
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
    get_header_with_actions(title, background_color, Vec::new())
}

/// Retrieve a header component with optional trailing action widgets (e.g. icon buttons).
///
/// The actions are placed after a flexible spacer so they align to the right edge of the header.
///
/// # Arguments
///
/// * `title` - The title to be displayed in the header
/// * `background_color` - The background color of the header
/// * `actions` - Trailing widgets rendered right-aligned in the header
///
/// # Returns
///
/// A Column widget representing the header.
pub fn get_header_with_actions<'a>(
    title: String,
    background_color: Color,
    actions: Vec<Element<'a, Message>>,
) -> Column<'a, Message> {
    let mut inner: Row<'a, Message> = row![
        text(title)
            .size(20)
            .width(Length::Shrink)
            .color(color!(255, 255, 255)),
        space::horizontal().width(Length::Fill),
    ];

    for action in actions {
        inner = inner.push(action);
    }

    iced::widget::column![row![
        container(iced::widget::column![inner])
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
    ]]
}
