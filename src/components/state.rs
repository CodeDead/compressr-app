use crate::components::settings::Settings;
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
    pub status: String,
    pub last_error_message: Option<String>,
    pub settings: Settings,
    pub update_version: Option<String>,
    pub update_download_url: Option<String>,
    pub update_info_url: Option<String>,
}

impl Default for State {
    /// Provides default values for the application state.
    ///
    /// # Returns
    ///
    /// A State instance with default values.
    fn default() -> Self {
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
            status: String::new(),
            last_error_message: None,
            settings: Settings::load_from_file(),
            update_version: None,
            update_download_url: None,
            update_info_url: None,
        }
    }
}
