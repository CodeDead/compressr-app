mod components;
mod services;
mod views;

use crate::views::main_view::MainView;
use iced::window::Position;
use iced::window::settings::PlatformSpecific;
use iced::{Size, Theme, window};

fn main() -> iced::Result {
    let icon_bytes = include_bytes!("../resources/compressr.png");
    let image = image::load_from_memory(icon_bytes).unwrap();
    let window_icon = window::icon::from_rgba(image.as_bytes().to_vec(), 256, 256)
        .expect("Failed to load window icon");

    let settings = window::Settings {
        size: Size::new(650.0, 400.0),
        resizable: true,
        position: Position::Centered,
        transparent: true,
        decorations: true,
        blur: true,
        icon: Some(window_icon),
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific {
            application_id: "com.codedead.compressr".to_string(),
            ..PlatformSpecific::default()
        },
        ..window::Settings::default()
    };

    iced::application(MainView::default, MainView::update, MainView::view)
        .theme(Theme::Oxocarbon)
        .centered()
        .window(settings)
        .title("Compressr")
        .run()
}
