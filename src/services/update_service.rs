use crate::models::version::Version;
use log::error;
use std::process::Command;

#[derive(Clone)]
pub struct UpdateService {
    update_server: String,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub semver: String,
    pub download_url: String,
    pub info_url: Option<String>,
}

impl UpdateService {
    /// Initialize a new UpdateService
    ///
    /// # Arguments
    ///
    /// * `update_server` - The URL of the update server to check for updates.
    ///
    /// # Returns
    ///
    /// A new instance of UpdateService
    pub fn new(update_server: String) -> Self {
        Self { update_server }
    }

    /// Checks for updates by fetching version information from the update server and comparing it with the current version.
    ///
    /// # Arguments
    ///
    /// * `current_semver` - The current version of the application in semantic versioning format.
    /// * `platform` - The platform of the application (e.g., "windows", "macos", "linux").
    /// * `arch` - The architecture of the application (e.g., "x64", "aarch64").
    ///
    /// # Returns
    ///
    /// Ok(Some(UpdateInfo)) if an update is available for the specified platform and architecture, Ok(None) if no update is available, or an error if the update check fails.
    pub async fn check_for_updates(
        &self,
        current_semver: String,
        platform: String,
        arch: String,
    ) -> Result<Option<UpdateInfo>, String> {
        let response = reqwest::get(&self.update_server)
            .await
            .map_err(|e| format!("Failed to fetch version info: {e}"))?;
        if response.status().is_success() {
            let version_info: Version = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse version info: {e}"))?;

            let version_parsed =
                semver::Version::parse(&version_info.semver.trim_start_matches('v'))
                    .map_err(|e| format!("Failed to parse version semver: {e}"))?;
            let current_semver_parsed =
                semver::Version::parse(&current_semver.trim_start_matches('v'))
                    .map_err(|e| format!("Failed to parse current version semver: {e}"))?;

            if version_parsed > current_semver_parsed {
                // Find the platform-specific download info
                if let Some(platform_info) = version_info
                    .platforms
                    .iter()
                    .find(|p| p.name == platform && p.arch == arch)
                {
                    Ok(Some(UpdateInfo {
                        semver: version_info.semver,
                        download_url: platform_info.download_url.clone(),
                        info_url: platform_info.info_url.clone(),
                    }))
                } else {
                    Ok(None) // No update available for this platform/arch
                }
            } else {
                Ok(None) // No update available
            }
        } else {
            Err(format!(
                "Failed to fetch version info: HTTP {}",
                response.status()
            ))
        }
    }

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
}
