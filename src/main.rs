#![windows_subsystem = "windows"]

mod components;
mod models;
mod services;
mod views;

use crate::components::app::App;
use log::info;

/// The main entry point of the application, initializing the logger and running the Iced application with the specified configuration.
///
/// # Returns
///
/// An Iced Result indicating the success or failure of running the application.
fn main() -> iced::Result {
    env_logger::init();

    info!("Starting Compressr");

    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .scale_factor(App::scale_factor)
        .run()
}

/// Get the current platform as a string, used for update checks and other platform-specific logic.
///
/// # Returns
///
/// A `String` representing the current platform, which can be "windows", "macos", or "linux".
pub(crate) fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
    .to_string()
}
