use iced::Theme;

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

impl Window {
    /// Initialize a new Window.
    ///
    /// # Arguments
    ///
    /// - `title` - The title of the window.
    /// - `kind` - The kind of window to create.
    /// - `theme` - The theme to use for the window.
    ///
    /// # Returns
    ///
    /// A new instance of `Window`.
    pub fn new(title: String, kind: WindowKind, theme: Theme) -> Self {
        Self { title, theme, kind }
    }
}
