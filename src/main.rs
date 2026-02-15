mod services;
mod views;

use crate::views::main_view::MainView;
use iced::window::Position;
use iced::{Size, Theme, window};

fn main() -> iced::Result {
    let settings = window::Settings {
        size: Size::new(550.0, 400.0),
        resizable: true,
        position: Position::Centered,
        transparent: true,
        decorations: true,
        blur: true,
        ..window::Settings::default()
    };

    iced::application(MainView::default, MainView::update, MainView::view)
        .theme(Theme::Oxocarbon)
        .centered()
        .window(settings)
        .title("Compressr")
        .run()
}
