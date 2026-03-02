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
            update_server: "https://codedead.com/Software/compressr/version.json".to_string(),
            theme: Some(Theme::Oxocarbon.to_string()),
            delete_files_after_compression: false,
        };

        // Save the default settings to a config file
        match serde_json::to_string_pretty(&res) {
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
        match serde_json::to_string(self) {
            Ok(json) => {
                if let Err(e) = fs::write("config.json", json) {
                    error!("Failed to write settings to config.json: {e}");
                };
            }
            Err(e) => {
                error!("Failed to serialize settings: {e}");
            }
        };
    }
}
