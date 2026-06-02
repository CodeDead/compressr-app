#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

/// Determines the target platform for which the code is being compiled.
///
/// # Returns
///
/// A string slice (`&'static str`) indicating the platform:
/// - `"windows"`: If the target operating system is Windows.
/// - `"macos"`: If the target operating system is macOS.
/// - `"linux"`: If the target operating system is Linux or another Unix-like system.
pub(crate) fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
}
