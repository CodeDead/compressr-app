pub struct Settings {
    pub auto_update: bool,
    pub update_server: String,
    pub theme: String,
    pub delete_files_after_compression: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_update: true,
            update_server: "https://codedead.com/Software/compressr/version.json".to_string(),
            theme: "Oxocarbon".to_string(),
            delete_files_after_compression: false,
        }
    }
}

impl Settings {
    /// Initialize a new Settings.
    ///
    /// # Arguments
    ///
    /// * `auto_update`: Whether to enable auto-update.
    /// * `update_server`: The URL of the update server.
    /// * `theme`: The theme name of the application.
    /// * `delete_files_after_compression`: Whether to delete original files after compression.
    ///
    /// # Returns
    ///
    /// A new instance of Settings.
    pub fn new(
        auto_update: bool,
        update_server: String,
        theme: String,
        delete_files_after_compression: bool,
    ) -> Self {
        Self {
            auto_update,
            update_server,
            theme,
            delete_files_after_compression,
        }
    }
}
