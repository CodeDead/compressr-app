use crate::services::theme_service::string_to_theme;
use iced::Theme;
use log::error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs;

const CONFIG_PATH: &str = "config.json";
const CONFIG_TMP_PATH: &str = "config.json.tmp";

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
}

impl Default for Settings {
    fn default() -> Self {
        let settings = Self {
            auto_update: true,
            update_server:
                "https://api.codedead.com/api/v1/version/4b66935d-7234-4662-aadc-3a12c1da621a"
                    .to_string(),
            theme: Theme::Oxocarbon,
            delete_files_after_compression: false,
            language_key: "en_US".to_string(),
            preserve_exif: false,
        };

        // Persist the defaults immediately so subsequent loads succeed.
        settings.save();

        settings
    }
}

impl Settings {
    /// Loads settings from `config.json`, falling back to defaults on any error.
    pub fn load_from_file() -> Self {
        match fs::read_to_string(CONFIG_PATH) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_else(|e| {
                error!("Failed to deserialize {CONFIG_PATH}: {e}");
                Self::default()
            }),
            Err(e) => {
                error!("Failed to read {CONFIG_PATH}: {e}");
                Self::default()
            }
        }
    }

    /// Atomically saves the current settings to `config.json`.
    pub fn save(&self) {
        let json = match serde_json::to_string(self) {
            Ok(j) => j,
            Err(e) => {
                error!("Failed to serialize settings: {e}");
                return;
            }
        };

        if let Err(e) = fs::write(CONFIG_TMP_PATH, &json) {
            error!("Failed to write settings to {CONFIG_TMP_PATH}: {e}");
            return;
        }

        if let Err(e) = fs::rename(CONFIG_TMP_PATH, CONFIG_PATH) {
            #[cfg(target_os = "windows")]
            {
                // On Windows, rename fails if the target already exists.
                if let Err(remove_err) = fs::remove_file(CONFIG_PATH) {
                    if remove_err.kind() != std::io::ErrorKind::NotFound {
                        error!("Failed to remove existing {CONFIG_PATH}: {remove_err}");
                        let _ = fs::remove_file(CONFIG_TMP_PATH);
                        return;
                    }
                }
                if let Err(rename_err) = fs::rename(CONFIG_TMP_PATH, CONFIG_PATH) {
                    error!("Failed to rename {CONFIG_TMP_PATH} to {CONFIG_PATH}: {rename_err}");
                    let _ = fs::remove_file(CONFIG_TMP_PATH);
                }
            }

            #[cfg(not(target_os = "windows"))]
            {
                error!("Failed to rename {CONFIG_TMP_PATH} to {CONFIG_PATH}: {e}");
                let _ = fs::remove_file(CONFIG_TMP_PATH);
            }
        }
    }
}
