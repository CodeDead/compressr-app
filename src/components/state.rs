use crate::components::settings::Settings;
use crate::models::language::Language;
use crate::services::image_service::OutputFormat;

pub struct State {
    pub input_path: Vec<String>,
    pub output_path: String,
    pub scale: u32,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub quality: u8,
    pub format: OutputFormat,
    pub is_compressing: bool,
    pub compression_succeeded: bool,
    pub show_no_update_view: bool,
    pub last_error_message: Option<String>,
    pub settings: Settings,
    pub update_version: Option<String>,
    pub update_download_url: Option<String>,
    pub update_info_url: Option<String>,
    pub languages: Vec<Language>,
}

impl Default for State {
    /// Provides default values for the application state.
    ///
    /// # Returns
    ///
    /// A State instance with default values.
    fn default() -> Self {
        let en_us_bytes = include_bytes!("../../languages/en_us.json");
        let fr_fr_bytes = include_bytes!("../../languages/fr_fr.json");
        let nl_nl_bytes = include_bytes!("../../languages/nl_nl.json");
        let ru_ru_bytes = include_bytes!("../../languages/ru_ru.json");

        let en_us_string = String::from_utf8_lossy(en_us_bytes).to_string();
        let fr_fr_string = String::from_utf8_lossy(fr_fr_bytes).to_string();
        let nl_nl_string = String::from_utf8_lossy(nl_nl_bytes).to_string();
        let ru_ru_string = String::from_utf8_lossy(ru_ru_bytes).to_string();

        let en_us_language =
            serde_json::from_str::<Language>(&en_us_string).unwrap_or_else(|err| {
                panic!("Failed to deserialize en_US language file: {err}");
            });
        let fr_fr_language =
            serde_json::from_str::<Language>(&fr_fr_string).unwrap_or_else(|err| {
                panic!("Failed to deserialize fr_FR language file: {err}");
            });
        let nl_nl_language =
            serde_json::from_str::<Language>(&nl_nl_string).unwrap_or_else(|err| {
                panic!("Failed to deserialize nl_NL language file: {err}");
            });
        let ru_ru_language =
            serde_json::from_str::<Language>(&ru_ru_string).unwrap_or_else(|err| {
                panic!("Failed to deserialize ru_RU language file: {err}");
            });

        let languages = vec![
            en_us_language,
            fr_fr_language,
            nl_nl_language,
            ru_ru_language,
        ];

        State {
            input_path: Vec::new(),
            output_path: String::new(),
            scale: 100,
            height: None,
            width: None,
            quality: 100,
            format: OutputFormat::Jpeg,
            is_compressing: false,
            compression_succeeded: false,
            show_no_update_view: false,
            last_error_message: None,
            settings: Settings::load_from_file(),
            update_version: None,
            update_download_url: None,
            update_info_url: None,
            languages,
        }
    }
}
