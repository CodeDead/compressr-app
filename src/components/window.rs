use crate::components::app::Message;
use crate::components::state::State;
use crate::models::language::Language;
use crate::views::{
    about_view, error_view, main_view, no_update_view, results_view, settings_view, update_view,
};
use iced::window::Position;
#[cfg(target_os = "linux")]
use iced::window::settings::PlatformSpecific;
use iced::{Element, Size, Theme, window};

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub theme: Theme,
    pub kind: WindowKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Main,
    Settings,
    Update,
    Error,
    About,
    NoUpdate,
    Results,
}

impl WindowKind {
    /// Returns the default `(width, height)` in pixels for a window of this kind.
    ///
    /// Centralizes the per-window dimensions that callers previously hardcoded at each
    /// open site.
    ///
    /// # Returns
    ///
    /// The default window size in pixels as a tuple `(width, height)`.
    pub fn default_size(self) -> (f32, f32) {
        match self {
            WindowKind::Main => (650.0, 385.0),
            WindowKind::Settings => (500.0, 400.0),
            WindowKind::Update => (400.0, 190.0),
            WindowKind::Error => (400.0, 210.0),
            WindowKind::About => (450.0, 270.0),
            WindowKind::NoUpdate => (400.0, 180.0),
            WindowKind::Results => (600.0, 400.0),
        }
    }

    /// Returns the localized window title for this kind, given the active language.
    ///
    /// # Arguments
    ///
    /// * `language` - The active language for localization.
    ///
    /// # Returns
    ///
    /// The localized window title, based on the provided language.
    pub fn title(self, language: &Language) -> String {
        match self {
            WindowKind::Main => "Compressr".to_string(),
            WindowKind::Settings => language.compressr_settings.clone(),
            WindowKind::Update => language.compressr_update.clone(),
            WindowKind::Error => language.compressr_error.clone(),
            WindowKind::About => language.compressr_about.clone(),
            WindowKind::NoUpdate => language.compressr_no_update.clone(),
            WindowKind::Results => language.compressr_results.clone(),
        }
    }

    /// Renders the view associated with this window kind.
    ///
    /// # Arguments
    ///
    /// * `state` - The application state used to render the view.
    ///
    /// # Returns
    ///
    /// The rendered view element for the window kind.
    pub fn view(self, state: &State) -> Element<'_, Message> {
        match self {
            WindowKind::Main => main_view::view(state),
            WindowKind::Settings => settings_view::view(state),
            WindowKind::Update => update_view::view(state),
            WindowKind::Error => error_view::view(state),
            WindowKind::About => about_view::view(state),
            WindowKind::NoUpdate => no_update_view::view(state),
            WindowKind::Results => results_view::view(state),
        }
    }
}

impl Window {
    /// Initialize a new Window.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the window.
    /// * `kind` - The kind of window to create.
    /// * `theme` - The theme to use for the window.
    ///
    /// # Returns
    ///
    /// A new instance of `Window`.
    pub fn new(title: String, kind: WindowKind, theme: Theme) -> Self {
        Self { title, theme, kind }
    }
}

/// Loads the application icon from embedded PNG bytes.
///
/// # Returns
///
/// An `Icon` object representing the application icon, ready to be used in window settings.
/// If the icon fails to load, the function will panic with an error message.
pub fn load_app_icon() -> window::icon::Icon {
    let bytes = include_bytes!("../../resources/compressr.png");
    let img = image::load_from_memory(bytes)
        .expect("Failed to load application icon from embedded PNG bytes")
        .into_rgba8();
    let (w, h) = (img.width(), img.height());
    window::icon::from_rgba(img.into_raw(), w, h).expect("Failed to load window icon")
}

/// Builds [`window::Settings`] for the given pixel size and icon.
///
/// # Arguments
///
/// * `size` - A tuple containing the width and height of the window in pixels.
/// * `icon` - An `Icon` object to be used as the window's icon.
///
/// # Returns
///
/// A `window::Settings` object, configured with the specified size, icon, and other properties such as transparency, decorations, and blur.
/// The settings also include platform-specific configuration for Linux to set the application ID.
/// This function centralizes the window configuration to ensure consistency across different windows in the application.
pub fn make_window_settings(size: (f32, f32), icon: window::icon::Icon) -> window::Settings {
    window::Settings {
        size: Size::new(size.0, size.1),
        resizable: true,
        position: Position::Centered,
        transparent: true,
        decorations: true,
        blur: true,
        icon: Some(icon),
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific {
            application_id: "com.codedead.compressr".to_string(),
            ..PlatformSpecific::default()
        },
        ..window::Settings::default()
    }
}
