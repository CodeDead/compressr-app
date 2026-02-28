use crate::services::image_service::OutputFormat;

pub struct State {
    pub input_path: String,
    pub output_path: String,
    pub scale: u32,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub quality: u8,
    pub format: OutputFormat,
    pub is_compressing: bool,
    pub compression_succeeded: bool,
    pub status: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            input_path: String::new(),
            output_path: String::new(),
            scale: 100,
            height: None,
            width: None,
            quality: 100,
            format: OutputFormat::Jpeg,
            is_compressing: false,
            compression_succeeded: false,
            status: String::new(),
        }
    }
}
