use iced::Theme;
use log::error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub auto_update: bool,
    pub update_server: String,
    pub theme: Option<String>,
    pub delete_files_after_compression: bool,
    pub language_key: String,
    pub preserve_exif: bool,
}

impl Default for Settings {
    /// Provides default settings for the application.
    ///
    /// # Returns
    ///
    /// A Settings instance with default values.
    fn default() -> Self {
        let res = Self {
            auto_update: true,
            update_server:
                "https://api.codedead.com/api/v1/version/4b66935d-7234-4662-aadc-3a12c1da621a"
                    .to_string(),
            theme: Some(Theme::Oxocarbon.to_string()),
            delete_files_after_compression: false,
            language_key: String::from("en_US"),
            preserve_exif: false,
        };

        // Save the default settings to a config file
        match serde_json::to_string(&res) {
            Ok(json) => {
                if let Err(e) = fs::write("config.json", json) {
                    error!("Failed to write default settings to config.json: {e}");
                };
            }
            Err(e) => {
                error!("Failed to serialize default settings: {e}");
            }
        };

        res
    }
}

impl Settings {
    /// Loads settings from a config file. If the file does not exist or fails to read, it returns default settings.
    ///
    /// # Returns
    ///
    /// A Settings instance loaded from the config file or default settings if loading fails.
    pub fn load_from_file() -> Self {
        match fs::read_to_string("config.json") {
            Ok(json) => serde_json::from_str(&json).unwrap_or_else(|e| {
                error!("Failed to deserialize settings from config.json: {e}");
                Self::default()
            }),
            Err(e) => {
                error!("Failed to read config.json: {e}");
                Self::default()
            }
        }
    }

    /// Saves the current settings to a config file. If serialization or file writing fails, it logs an error.
    pub fn save(&self) {
        let temp_path = "config.json.tmp";
        let target_path = "config.json";

        match serde_json::to_string(self) {
            Ok(json) => {
                if let Err(e) = fs::write(temp_path, json) {
                    error!("Failed to write settings to temporary file: {e}");
                    return;
                }

                if let Err(e) = fs::rename(temp_path, target_path) {
                    #[cfg(target_os = "windows")]
                    {
                        if let Err(remove_err) = fs::remove_file(target_path)
                            && remove_err.kind() != std::io::ErrorKind::NotFound
                        {
                            error!("Failed to replace existing config.json: {remove_err}");
                            let _ = fs::remove_file(temp_path);
                            return;
                        }

                        if let Err(rename_err) = fs::rename(temp_path, target_path) {
                            error!("Failed to rename temporary file to config.json: {rename_err}");
                            let _ = fs::remove_file(temp_path);
                        }
                    }

                    #[cfg(not(target_os = "windows"))]
                    {
                        error!("Failed to rename temporary file to config.json: {e}");
                        let _ = fs::remove_file(temp_path);
                    }
                }
            }
            Err(e) => {
                error!("Failed to serialize settings: {e}");
            }
        }
    }
}
