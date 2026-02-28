use crate::components::app::Message;
use crate::components::state::State;
use iced::widget::{container, row, space, text};
use iced::{Element, Length, color};

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

    container(header)
        .width(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
