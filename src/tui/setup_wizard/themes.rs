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
    /// RGB color value for question prompts
    pub question: (u8, u8, u8),
    /// RGB color value for general statements
    pub statement: (u8, u8, u8),
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
                "question".to_string(),
                format!(
                    "{},{},{}",
                    self.question.0, self.question.1, self.question.2
                ),
            ),
            (
                "statement".to_string(),
                format!(
                    "{},{},{}",
                    self.statement.0, self.statement.1, self.statement.2
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

    /// Gets a Ratatui Style for the statement color.
    pub fn statement_style(&self) -> Style {
        Style::default().fg(Color::Rgb(
            self.statement.0,
            self.statement.1,
            self.statement.2,
        ))
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
    Theme {
        name: "Cappuccino",
        icon: None,
        scheme: ColorScheme {
            informational: (238, 212, 159),
            warning: (237, 135, 150),
            success: (166, 218, 149),
            question: (202, 211, 245),
            statement: (244, 219, 214),
        },
    },
    Theme {
        name: "Darcula",
        icon: None,
        scheme: ColorScheme {
            informational: (241, 250, 140),
            warning: (255, 85, 85),
            success: (80, 250, 123),
            question: (139, 233, 253),
            statement: (248, 248, 242),
        },
    },
    Theme {
        name: "GirlyPop",
        icon: Some("GirlyPop"),
        scheme: ColorScheme {
            informational: (237, 69, 146),
            warning: (241, 218, 165),
            success: (243, 214, 243),
            question: (255, 128, 177),
            statement: (255, 148, 219),
        },
    },
    Theme {
        name: "Autumnal Vibes",
        icon: None,
        scheme: ColorScheme {
            informational: (218, 165, 32),
            warning: (178, 34, 34),
            success: (189, 183, 107),
            question: (255, 140, 0),
            statement: (210, 105, 30),
        },
    },
    Theme {
        name: "Skeletal",
        icon: None,
        scheme: ColorScheme {
            informational: (248, 248, 240),
            warning: (255, 140, 0),
            success: (152, 251, 152),
            question: (138, 43, 226),
            statement: (211, 211, 211),
        },
    },
    Theme {
        name: "Default",
        icon: None,
        scheme: ColorScheme {
            informational: (255, 215, 0),
            warning: (255, 0, 0),
            success: (0, 255, 0),
            question: (255, 215, 0),
            statement: (255, 255, 255),
        },
    },
];

/// Creates a custom color scheme from RGB values.
pub fn create_custom_scheme(
    informational: (u8, u8, u8),
    warning: (u8, u8, u8),
    success: (u8, u8, u8),
    question: (u8, u8, u8),
    statement: (u8, u8, u8),
) -> ColorScheme {
    ColorScheme {
        informational,
        warning,
        success,
        question,
        statement,
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        THEMES[4].scheme.clone() // Default theme
    }
}
