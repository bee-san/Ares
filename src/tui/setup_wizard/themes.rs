//! Theme definitions for the setup wizard.
//!
//! This module contains all the predefined color schemes and the
//! structures for representing themes.

use ratatui::style::{Color, Style};

/// Represents a color scheme with RGB values for different message types.
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// RGB color value for informational messages
    pub informational: (u8, u8, u8),
    /// RGB color value for warning messages
    pub warning: (u8, u8, u8),
    /// RGB color value for success messages
    pub success: (u8, u8, u8),
    /// RGB color value for error messages
    pub error: (u8, u8, u8),
    /// RGB color value for question prompts
    pub question: (u8, u8, u8),
}

impl ColorScheme {
    /// Converts the scheme to a HashMap of "r,g,b" strings for config storage.
    pub fn to_config_strings(&self) -> Vec<(String, String)> {
        vec![
            (
                "informational".to_string(),
                format!(
                    "{},{},{}",
                    self.informational.0, self.informational.1, self.informational.2
                ),
            ),
            (
                "warning".to_string(),
                format!("{},{},{}", self.warning.0, self.warning.1, self.warning.2),
            ),
            (
                "success".to_string(),
                format!("{},{},{}", self.success.0, self.success.1, self.success.2),
            ),
            (
                "error".to_string(),
                format!("{},{},{}", self.error.0, self.error.1, self.error.2),
            ),
            (
                "question".to_string(),
                format!(
                    "{},{},{}",
                    self.question.0, self.question.1, self.question.2
                ),
            ),
        ]
    }

    /// Gets a Ratatui Style for the informational color.
    pub fn informational_style(&self) -> Style {
        Style::default().fg(Color::Rgb(
            self.informational.0,
            self.informational.1,
            self.informational.2,
        ))
    }

    /// Gets a Ratatui Style for the warning color.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(Color::Rgb(self.warning.0, self.warning.1, self.warning.2))
    }

    /// Gets a Ratatui Style for the success color.
    pub fn success_style(&self) -> Style {
        Style::default().fg(Color::Rgb(self.success.0, self.success.1, self.success.2))
    }

    /// Gets a Ratatui Style for the question color.
    pub fn question_style(&self) -> Style {
        Style::default().fg(Color::Rgb(
            self.question.0,
            self.question.1,
            self.question.2,
        ))
    }

    /// Gets a Ratatui Style for the error color.
    pub fn error_style(&self) -> Style {
        Style::default().fg(Color::Rgb(self.error.0, self.error.1, self.error.2))
    }

    /// Gets the primary accent color as a Ratatui Color.
    pub fn accent_color(&self) -> Color {
        Color::Rgb(
            self.informational.0,
            self.informational.1,
            self.informational.2,
        )
    }
}

/// A named theme with its color scheme.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Display name of the theme
    pub name: &'static str,
    /// The color scheme for this theme
    pub scheme: ColorScheme,
    /// Optional emoji/icon for the theme
    pub icon: Option<&'static str>,
}

/// All available themes.
pub static THEMES: &[Theme] = &[
    // === Original Themes ===
    Theme {
        name: "Cappuccino",
        icon: Some("☕"),
        scheme: ColorScheme {
            informational: (238, 212, 159),
            warning: (237, 135, 150),
            success: (166, 218, 149),
            error: (237, 135, 150),
            question: (202, 211, 245),
        },
    },
    Theme {
        name: "Darcula",
        icon: Some("🧛"),
        scheme: ColorScheme {
            informational: (241, 250, 140),
            warning: (255, 85, 85),
            success: (80, 250, 123),
            error: (255, 85, 85),
            question: (139, 233, 253),
        },
    },
    Theme {
        name: "GirlyPop",
        icon: Some("💅"),
        scheme: ColorScheme {
            informational: (237, 69, 146),
            warning: (241, 218, 165),
            success: (243, 214, 243),
            error: (241, 218, 165),
            question: (255, 128, 177),
        },
    },
    Theme {
        name: "Autumnal Vibes",
        icon: Some("🍂"),
        scheme: ColorScheme {
            informational: (218, 165, 32),
            warning: (178, 34, 34),
            success: (189, 183, 107),
            error: (178, 34, 34),
            question: (255, 140, 0),
        },
    },
    Theme {
        name: "Skeletal",
        icon: Some("💀"),
        scheme: ColorScheme {
            informational: (248, 248, 240),
            warning: (255, 140, 0),
            success: (152, 251, 152),
            error: (255, 140, 0),
            question: (138, 43, 226),
        },
    },
    Theme {
        name: "Default",
        icon: None,
        scheme: ColorScheme {
            informational: (255, 215, 0),
            warning: (255, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            question: (255, 215, 0),
        },
    },
    // === Cyberpunk & Neon Themes ===
    Theme {
        name: "Cyberpunk 2077",
        icon: Some("🌃"),
        scheme: ColorScheme {
            // Neon-soaked Night City aesthetic
            informational: (252, 255, 82), // Neon Yellow
            warning: (236, 28, 219),       // Hot Pink
            success: (0, 242, 234),        // Cyan
            error: (255, 36, 99),          // Bright Red
            question: (0, 194, 255),       // Electric Blue
        },
    },
    Theme {
        name: "Synthwave '84",
        icon: Some("🌆"),
        scheme: ColorScheme {
            // Retro 80s neon aesthetic
            informational: (255, 113, 206), // Neon Pink
            warning: (255, 0, 128),         // Hot Pink
            success: (0, 255, 255),         // Cyan
            error: (255, 16, 240),          // Magenta Red
            question: (179, 0, 255),        // Purple
        },
    },
    Theme {
        name: "Matrix",
        icon: Some("💊"),
        scheme: ColorScheme {
            // Classic green terminal aesthetic
            informational: (0, 255, 65), // Bright Green
            warning: (154, 255, 0),      // Yellow-Green
            success: (0, 255, 0),        // Pure Green
            error: (0, 200, 50),         // Dark Green
            question: (50, 255, 50),     // Lime Green
        },
    },
    // === Popular Editor Themes ===
    Theme {
        name: "Nord",
        icon: Some("🌲"),
        scheme: ColorScheme {
            // Cool-toned Arctic theme
            informational: (136, 192, 208), // Frost Blue
            warning: (191, 97, 106),        // Aurora Red
            success: (163, 190, 140),       // Aurora Green
            error: (191, 97, 106),          // Aurora Red
            question: (143, 188, 187),      // Frost Cyan
        },
    },
    Theme {
        name: "Dracula",
        icon: Some("🦇"),
        scheme: ColorScheme {
            // Purple-dominant theme
            informational: (189, 147, 249), // Purple
            warning: (255, 184, 108),       // Orange
            success: (80, 250, 123),        // Green
            error: (255, 85, 85),           // Red
            question: (255, 121, 198),      // Pink
        },
    },
    Theme {
        name: "Monokai",
        icon: Some("🎨"),
        scheme: ColorScheme {
            // Classic vibrant editor theme
            informational: (230, 219, 116), // Yellow
            warning: (253, 151, 31),        // Orange
            success: (166, 226, 46),        // Green
            error: (249, 38, 114),          // Pink
            question: (174, 129, 255),      // Purple
        },
    },
    Theme {
        name: "Gruvbox",
        icon: Some("📦"),
        scheme: ColorScheme {
            // Retro warm color palette
            informational: (250, 189, 47), // Yellow
            warning: (251, 73, 52),        // Red
            success: (184, 187, 38),       // Green
            error: (251, 73, 52),          // Red
            question: (254, 128, 25),      // Orange
        },
    },
    Theme {
        name: "Tokyo Night",
        icon: Some("🗼"),
        scheme: ColorScheme {
            // Cool blue VS Code theme
            informational: (125, 207, 255), // Blue
            warning: (247, 118, 142),       // Red
            success: (158, 206, 106),       // Green
            error: (247, 118, 142),         // Red
            question: (125, 207, 255),      // Cyan
        },
    },
    // === Additional Popular Themes ===
    Theme {
        name: "Solarized Dark",
        icon: Some("🌙"),
        scheme: ColorScheme {
            // Classic precision color theme
            informational: (181, 137, 0), // Yellow
            warning: (203, 75, 22),       // Orange
            success: (133, 153, 0),       // Green
            error: (220, 50, 47),         // Red
            question: (38, 139, 210),     // Blue
        },
    },
    Theme {
        name: "Solarized Light",
        icon: Some("☀️"),
        scheme: ColorScheme {
            // Classic light precision theme
            informational: (181, 137, 0), // Yellow
            warning: (203, 75, 22),       // Orange
            success: (133, 153, 0),       // Green
            error: (220, 50, 47),         // Red
            question: (38, 139, 210),     // Blue
        },
    },
    Theme {
        name: "One Dark",
        icon: Some("⚛️"),
        scheme: ColorScheme {
            // Atom's iconic dark theme
            informational: (229, 192, 123), // Yellow
            warning: (224, 108, 117),       // Red
            success: (152, 195, 121),       // Green
            error: (224, 108, 117),         // Red
            question: (97, 175, 239),       // Blue
        },
    },
    Theme {
        name: "Catppuccin Mocha",
        icon: Some("🐱"),
        scheme: ColorScheme {
            // Soothing pastel theme
            informational: (249, 226, 175), // Yellow (Rosewater)
            warning: (250, 179, 135),       // Peach
            success: (166, 227, 161),       // Green
            error: (243, 139, 168),         // Red (Maroon)
            question: (137, 180, 250),      // Blue (Sapphire)
        },
    },
    Theme {
        name: "Palenight",
        icon: Some("🌜"),
        scheme: ColorScheme {
            // Material palenight theme
            informational: (255, 203, 107), // Yellow
            warning: (247, 140, 108),       // Orange
            success: (195, 232, 141),       // Green
            error: (255, 85, 114),          // Red
            question: (130, 170, 255),      // Blue
        },
    },
    Theme {
        name: "Ayu Dark",
        icon: Some("🔆"),
        scheme: ColorScheme {
            // Ayu color scheme
            informational: (255, 180, 84), // Yellow/Orange
            warning: (255, 51, 51),        // Red
            success: (186, 230, 126),      // Green
            error: (255, 51, 51),          // Red
            question: (115, 184, 255),     // Blue
        },
    },
    Theme {
        name: "GitHub Dark",
        icon: Some("🐙"),
        scheme: ColorScheme {
            // GitHub's dark theme
            informational: (210, 168, 255), // Purple
            warning: (248, 81, 73),         // Red
            success: (63, 185, 80),         // Green
            error: (248, 81, 73),           // Red
            question: (121, 192, 255),      // Blue
        },
    },
    Theme {
        name: "Oceanic",
        icon: Some("🌊"),
        scheme: ColorScheme {
            // Deep sea inspired theme
            informational: (102, 217, 239), // Cyan
            warning: (255, 157, 0),         // Orange
            success: (166, 226, 46),        // Green
            error: (249, 38, 114),          // Pink/Red
            question: (102, 217, 239),      // Cyan
        },
    },
    Theme {
        name: "High Contrast",
        icon: Some("👁️"),
        scheme: ColorScheme {
            // Accessibility-focused high contrast
            informational: (255, 255, 255), // White
            warning: (255, 128, 0),         // Orange
            success: (0, 255, 0),           // Green
            error: (255, 0, 0),             // Red
            question: (0, 255, 255),        // Cyan
        },
    },
];

/// Creates a custom color scheme from RGB values.
pub fn create_custom_scheme(
    informational: (u8, u8, u8),
    warning: (u8, u8, u8),
    success: (u8, u8, u8),
    error: (u8, u8, u8),
    question: (u8, u8, u8),
) -> ColorScheme {
    ColorScheme {
        informational,
        warning,
        success,
        error,
        question,
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        THEMES[5].scheme.clone() // Default theme
    }
}
