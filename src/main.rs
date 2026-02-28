mod components;
mod services;
mod views;

use crate::components::app::App;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .scale_factor(App::scale_factor)
        .run()
}
