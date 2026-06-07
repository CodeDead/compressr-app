use crate::components::settings::Settings;
use crate::models::language::Language;
use crate::services::image_service::{CompressionResult, OutputFormat};
use iced::widget::image;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub struct MainViewIcons {
    pub settings: image::Handle,
    pub settings_dark: image::Handle,
    pub info: image::Handle,
    pub info_dark: image::Handle,
}

pub struct State {
    pub input_path: Vec<String>,
    pub output_path: String,
    pub scale: u32,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub quality: u8,
    pub format: OutputFormat,
    pub is_compressing: bool,
    pub compression_results: Vec<CompressionResult>,
    pub last_error_message: Option<String>,
    pub settings: Settings,
    pub update_version: Option<String>,
    pub update_download_url: Option<String>,
    pub update_info_url: Option<String>,
    pub languages: Vec<Language>,
    pub main_view_icons: MainViewIcons,
    pub show_input_dropdown: bool,
    pub progress_completed: usize,
    pub progress_total: usize,
    pub compression_aborted: Arc<AtomicBool>,
}

impl Default for State {
    /// Provides a blank application state using pure default settings (no file I/O).
    ///
    /// Prefer [`State::with_settings`] when you have already loaded settings from disk
    /// and need to preserve them.
    fn default() -> Self {
        Self::with_settings(Settings::default())
    }
}

impl State {
    /// Constructs a `State` using the provided `settings`, loading all embedded language
    /// files and icon assets.
    ///
    /// This is the primary constructor used by [`crate::components::app::App::new`], which
    /// first calls [`Settings::load_from_file`] to obtain the settings (and a flag indicating
    /// whether they were read from disk), then passes them here.
    ///
    /// # Arguments
    ///
    /// * `settings` - Pre-loaded (or default) application settings.
    ///
    /// # Returns
    ///
    /// A fully initialised `State`.
    pub fn with_settings(settings: Settings) -> Self {
        /// Loads and deserializes a language JSON file from embedded bytes.
        fn load_lang(bytes: &[u8], name: &str) -> Language {
            let json = std::str::from_utf8(bytes)
                .unwrap_or_else(|_| panic!("Language file '{name}' is not valid UTF-8"));
            serde_json::from_str::<Language>(json)
                .unwrap_or_else(|err| panic!("Failed to deserialize {name} language file: {err}"))
        }

        let languages = vec![
            load_lang(include_bytes!("../../languages/en_us.json"), "en_US"),
            load_lang(include_bytes!("../../languages/fr_fr.json"), "fr_FR"),
            load_lang(include_bytes!("../../languages/nl_nl.json"), "nl_NL"),
            load_lang(include_bytes!("../../languages/ru_ru.json"), "ru_RU"),
            load_lang(include_bytes!("../../languages/uk_ua.json"), "uk_UA"),
            load_lang(include_bytes!("../../languages/zh_cn.json"), "zh_CN"),
            load_lang(include_bytes!("../../languages/es_es.json"), "es_ES"),
            load_lang(include_bytes!("../../languages/pt_pt.json"), "pt_PT"),
            load_lang(include_bytes!("../../languages/ja_jp.json"), "ja_JP"),
        ];
        let main_view_icons = MainViewIcons {
            settings: image::Handle::from_bytes(
                include_bytes!("../../resources/settings.png").as_slice(),
            ),
            settings_dark: image::Handle::from_bytes(
                include_bytes!("../../resources/settings_dark.png").as_slice(),
            ),
            info: image::Handle::from_bytes(include_bytes!("../../resources/info.png").as_slice()),
            info_dark: image::Handle::from_bytes(
                include_bytes!("../../resources/info_dark.png").as_slice(),
            ),
        };

        State {
            input_path: Vec::new(),
            output_path: String::new(),
            scale: 100,
            height: None,
            width: None,
            quality: 100,
            format: OutputFormat::Jpeg,
            is_compressing: false,
            compression_results: Vec::new(),
            last_error_message: None,
            settings,
            update_version: None,
            update_download_url: None,
            update_info_url: None,
            languages,
            main_view_icons,
            show_input_dropdown: false,
            progress_completed: 0,
            progress_total: 0,
            compression_aborted: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Returns a reference to the language matching the current `language_key` setting,
    /// falling back to the first language if no match is found.
    ///
    /// # Returns
    ///
    /// A reference to the matching `Language` or the first language if no match is found.
    pub fn current_language(&self) -> &Language {
        self.languages
            .iter()
            .find(|l| l.language_key == self.settings.language_key)
            .unwrap_or(
                self.languages
                    .first()
                    .expect("At least one language should be defined!"),
            )
    }
}
