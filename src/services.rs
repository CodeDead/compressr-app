use log::error;
use reqwest::Url;
use std::process::Command;

pub(crate) mod folder_scanner;
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
    let parsed_url = Url::parse(url).map_err(|err| format!("Invalid URL: {err}"))?;
    if parsed_url.scheme() != "https" {
        return Err(format!(
            "Refusing to open non-HTTPS URL with scheme '{}'",
            parsed_url.scheme()
        ));
    }

    #[cfg(target_os = "windows")]
    let result = {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "start", "", url]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        cmd.spawn()
    };

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(url).spawn();

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let result = Command::new("xdg-open").arg(url).spawn();

    if let Err(err) = result {
        error!("Failed to open URL: {err}");
        return Err(format!("Failed to open URL: {err}"));
    }

    Ok(())
}
