use iced::Theme;

/// Converts a theme name string to the corresponding [`Theme`] variant.
/// Falls back to [`Theme::Oxocarbon`] for unrecognized names.
///
/// # Arguments
///
/// * `theme_str` - The display name of the theme to convert.
///
/// # Returns
///
/// The corresponding [`Theme`] variant, or [`Theme::Oxocarbon`] if the name is unrecognized.
pub fn string_to_theme(theme_str: &str) -> Theme {
    Theme::ALL
        .iter()
        .find(|t| t.to_string() == theme_str)
        .cloned()
        .unwrap_or(Theme::Oxocarbon)
}
