use crate::services::theme_service::string_to_theme;
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};
use iced::Theme;
use log::{error, info};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
static CONFIG_TMP_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Returns a reference to the configuration file's path as a static `PathBuf`.
///
/// # Returns
///
/// A reference to the configuration file's path.
fn config_path() -> &'static PathBuf {
    CONFIG_PATH.get_or_init(|| {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "CodeDead".to_string(),
            app_name: "Compressr".to_string(),
        })
        .expect("Failed to determine OS config directory");
        strategy.in_config_dir("config.json")
    })
}

/// Returns a reference to the temporary config file path as a static `PathBuf`.
///
/// The path is derived by appending `".tmp"` to the config file path, enabling
/// an atomic write-then-rename save strategy.
///
/// # Returns
///
/// A reference to the temporary configuration file's path.
fn config_tmp_path() -> &'static PathBuf {
    CONFIG_TMP_PATH.get_or_init(|| {
        let mut s = config_path().as_os_str().to_os_string();
        s.push(".tmp");
        PathBuf::from(s)
    })
}

/// Serializes a [`Theme`] as its display name string.
fn serialize_theme<S: Serializer>(theme: &Theme, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&theme.to_string())
}

/// Deserializes a [`Theme`] from its display name string, falling back to `Oxocarbon`.
fn deserialize_theme<'de, D: Deserializer<'de>>(d: D) -> Result<Theme, D::Error> {
    let s = String::deserialize(d)?;
    Ok(string_to_theme(&s))
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub auto_update: bool,
    pub update_server: String,
    #[serde(
        serialize_with = "serialize_theme",
        deserialize_with = "deserialize_theme"
    )]
    pub theme: Theme,
    pub delete_files_after_compression: bool,
    pub language_key: String,
    pub preserve_exif: bool,
    pub show_compression_results: bool,
    pub recursive_folder_scan: bool,
}

impl Default for Settings {
    /// Returns the default settings configuration.
    ///
    /// # Returns
    ///
    /// The default settings configuration.
    fn default() -> Self {
        Self {
            auto_update: true,
            update_server:
                "https://api.codedead.com/api/v1/version/4b66935d-7234-4662-aadc-3a12c1da621a"
                    .to_string(),
            theme: Theme::Oxocarbon,
            delete_files_after_compression: false,
            language_key: "en_US".to_string(),
            preserve_exif: false,
            show_compression_results: true,
            recursive_folder_scan: true,
        }
    }
}

impl Settings {
    /// Loads settings from the OS config directory, falling back to defaults on any error.
    ///
    /// # Returns
    ///
    /// A tuple of `(settings, loaded_from_file)` where `loaded_from_file` is `true` when
    /// settings were successfully read and parsed from disk, and `false` when defaults were
    /// used (first run, missing file, or corrupt/unreadable config). Callers should persist
    /// the settings when `loaded_from_file` is `false`.
    pub fn load_from_file() -> (Self, bool) {
        let path = config_path();
        match fs::read_to_string(path) {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(settings) => (settings, true),
                Err(e) => {
                    error!(
                        "Failed to deserialize {} - reverting to defaults: {e}",
                        path.display()
                    );
                    (Self::default(), false)
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!(
                    "No config file found at {} — using defaults",
                    path.display()
                );
                (Self::default(), false)
            }
            Err(e) => {
                error!(
                    "Failed to read {} - reverting to defaults: {e}",
                    path.display()
                );
                (Self::default(), false)
            }
        }
    }

    /// Atomically saves settings to the OS config directory.
    ///
    /// Writes to a temporary sibling file first, then renames the file for atomicity.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of the save operation.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = config_path();
        let tmp_path = config_tmp_path();

        if let Some(parent) = path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            error!(
                "Failed to create config directory {}: {e}",
                parent.display()
            );
            return Err(e);
        }

        let json = match serde_json::to_string(self) {
            Ok(j) => j,
            Err(e) => {
                error!("Failed to serialize settings: {e}");
                return Err(std::io::Error::other(e));
            }
        };

        if let Err(e) = fs::write(tmp_path, &json) {
            error!("Failed to write settings to {}: {e}", tmp_path.display());
            let _ = fs::remove_file(tmp_path);
            return Err(e);
        }

        #[cfg(target_os = "windows")]
        {
            let _ = fs::remove_file(path);
        }
        if let Err(e) = fs::rename(tmp_path, path) {
            error!(
                "Failed to rename {} to {}: {e}",
                tmp_path.display(),
                path.display()
            );
            let _ = fs::remove_file(tmp_path);
            return Err(e);
        }

        Ok(())
    }
}
