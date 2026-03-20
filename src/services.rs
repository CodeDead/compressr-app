use log::error;
use std::process::Command;

pub(crate) mod image_service;
pub(crate) mod theme_service;
pub(crate) mod update_service;

/// Opens the specified URL in the default web browser.
///
/// # Arguments
///
/// * `url` - The URL to open in the web browser.
///
/// # Returns
///
/// Ok(()) if the URL was successfully opened, or an error message if it failed.
pub fn open_website(url: &str) -> Result<(), String> {
    let result = match crate::get_platform().as_str() {
        "windows" => Command::new("cmd").args(["/C", "start", "", url]).spawn(),
        "macos" => Command::new("open").arg(url).spawn(),
        "linux" => Command::new("xdg-open").arg(url).spawn(),
        _ => {
            return Err(format!("Unsupported platform {}", std::env::consts::OS));
        }
    };

    if let Err(err) = result {
        error!("Failed to open URL: {err}");
        return Err(format!("Failed to open URL: {err}"));
    }

    Ok(())
}
