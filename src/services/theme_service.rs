use iced::Theme;

pub struct ThemeService;

impl ThemeService {
    /// Converts a theme name string to a Theme enum variant.
    ///
    /// # Arguments
    ///
    /// * `theme_str`: A string slice representing the theme name.
    ///
    /// # Returns
    ///
    /// A Theme enum variant corresponding to the provided theme name. If the theme name is not recognized, it defaults to Theme::Dark.
    pub fn string_to_theme(theme_str: &str) -> Theme {
        match theme_str {
            "Light" => Theme::Light,
            "Dark" => Theme::Dark,
            "Dracula" => Theme::Dracula,
            "Nord" => Theme::Nord,
            "Solarized Light" => Theme::SolarizedLight,
            "Solarized Dark" => Theme::SolarizedDark,
            "Gruvbox Light" => Theme::GruvboxLight,
            "Gruvbox Dark" => Theme::GruvboxDark,
            "Catppuccin Latte" => Theme::CatppuccinLatte,
            "Catppuccin Frappé" => Theme::CatppuccinFrappe,
            "Catppuccin Macchiato" => Theme::CatppuccinMacchiato,
            "Catppuccin Mocha" => Theme::CatppuccinMocha,
            "Tokyo Night" => Theme::TokyoNight,
            "Tokyo Night Storm" => Theme::TokyoNightStorm,
            "Tokyo Night Light" => Theme::TokyoNightLight,
            "Kanagawa Wave" => Theme::KanagawaWave,
            "Kanagawa Dragon" => Theme::KanagawaDragon,
            "Kanagawa Lotus" => Theme::KanagawaLotus,
            "Moonfly" => Theme::Moonfly,
            "Nightfly" => Theme::Nightfly,
            "Oxocarbon" => Theme::Oxocarbon,
            "Ferra" => Theme::Ferra,
            _ => Theme::Dark, // Default fallback
        }
    }
}
