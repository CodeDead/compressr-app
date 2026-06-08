use crate::models::version::Version;
use std::time::Duration;

#[derive(Clone)]
pub struct UpdateService {
    update_server: String,
    client: reqwest::Client,
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
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create update HTTP client");

        Self {
            update_server,
            client,
        }
    }

    /// Checks for updates by fetching version information from the update server and comparing it with the current version.
    ///
    /// The current platform (`std::env::consts::OS`) and architecture (`std::env::consts::ARCH`) are
    /// resolved internally and matched against the platform-specific download entries.
    ///
    /// # Arguments
    ///
    /// * `current_semver` - The current version of the application in semantic versioning format.
    ///
    /// # Returns
    ///
    /// Ok(Some(UpdateInfo)) if an update is available for the current platform and architecture, Ok(None) if no update is available, or an error if the update check fails.
    pub async fn check_for_updates(
        &self,
        current_semver: String,
    ) -> Result<Option<UpdateInfo>, String> {
        let platform = std::env::consts::OS;
        let arch = std::env::consts::ARCH;

        let response = self
            .client
            .get(&self.update_server)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch version info: {e}"))?;
        if response.status().is_success() {
            let response_bytes = response
                .bytes()
                .await
                .map_err(|e| format!("Failed to read version info: {e}"))?;

            let version_info: Version = serde_json::from_slice(&response_bytes)
                .map_err(|e| format!("Failed to parse version info: {e}"))?;

            let version_parsed =
                semver::Version::parse(version_info.semver.trim_start_matches('v'))
                    .map_err(|e| format!("Failed to parse version semver: {e}"))?;
            let current_semver_parsed =
                semver::Version::parse(current_semver.trim_start_matches('v'))
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
}
