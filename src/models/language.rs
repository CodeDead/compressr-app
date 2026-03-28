use serde::Deserialize;

#[derive(Deserialize)]
pub struct Language {
    #[serde(rename = "languageKey")]
    pub language_key: String,
    #[serde(rename = "languageName")]
    pub language_name: String,
    pub input: String,
    pub output: String,
    pub browse: String,
    pub format: String,
    pub quality: String,
    pub scale: String,
    pub width: String,
    pub height: String,
    pub compress: String,
    pub compressing: String,
    pub compressed: String,
    #[serde(rename = "latestVersionInstalled")]
    pub latest_version_installed: String,
    #[serde(rename = "compressrAbout")]
    pub compressr_about: String,
    #[serde(rename = "compressrSettings")]
    pub compressr_settings: String,
    #[serde(rename = "compressrError")]
    pub compressr_error: String,
    #[serde(rename = "compressrUpdate")]
    pub compressr_update: String,
    #[serde(rename = "aboutText")]
    pub about_text: String,
    pub website: String,
    pub donate: String,
    pub copy: String,
    pub close: String,
    #[serde(rename = "automaticallyCheckForUpdates")]
    pub automatically_check_for_updates: String,
    #[serde(rename = "deleteOriginalFilesAfterCompression")]
    pub delete_original_files_after_compression: String,
    #[serde(rename = "preserveExifData")]
    pub preserve_exif_data: String,
    pub theme: String,
    #[serde(rename = "selectTheme")]
    pub select_theme: String,
    pub language: String,
    #[serde(rename = "selectLanguage")]
    pub select_language: String,
    #[serde(rename = "checkForUpdates")]
    pub check_for_updates: String,
    #[serde(rename = "resetAllSettings")]
    pub reset_all_settings: String,
    #[serde(rename = "newVersionAvailable")]
    pub new_version_available: String,
    pub information: String,
    pub download: String,
    #[serde(rename = "unknownError")]
    pub unknown_error: String,
}
