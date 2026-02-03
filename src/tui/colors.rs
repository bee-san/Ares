//! TUI color scheme integration for Ciphey.
//!
//! This module provides a bridge between Ciphey's configuration-based colorscheme
//! and Ratatui's styling system. It converts the RGB color strings stored in
//! [`Config::colourscheme`](crate::config::Config) into Ratatui [`Style`] objects.

use ratatui::style::{Color, Modifier, Style};

use crate::config::Config;

/// TUI color styles derived from Ciphey's configuration.
///
/// This struct holds pre-computed [`Style`] objects for consistent styling
/// across the TUI. Styles are derived from the config's colorscheme when
/// available, with sensible fallbacks for missing values.
#[derive(Debug, Clone)]
pub struct TuiColors {
    /// Primary accent color for important UI elements.
    /// Derived from the "informational" colorscheme key.
    pub primary: Style,

    /// Style for success messages and indicators.
    /// Derived from the "success" colorscheme key.
    pub success: Style,

    /// Style for warning messages.
    /// Derived from the "warning" colorscheme key.
    pub warning: Style,

    /// Style for error messages.
    /// Derived from the "error" colorscheme key, falls back to red.
    pub error: Style,

    /// Style for normal text content.
    /// Uses the default terminal foreground color (white/light).
    pub text: Style,

    /// Style for dimmed or secondary text (arrows, ellipsis, etc).
    pub text_dimmed: Style,

    /// Style for dimmed or secondary text.
    /// Uses a gray color for less prominent information.
    pub muted: Style,

    /// Style for highlighted or selected items.
    /// Derived from "informational" with bold modifier.
    pub highlight: Style,

    /// Style for borders and dividers.
    /// Uses a subtle gray color.
    pub border: Style,

    /// Accent color for selected/highlighted items.
    pub accent: Style,

    /// Style for titles in panels and boxes.
    pub title: Style,

    /// Style for labels (e.g., "Decoder:", "Key:").
    pub label: Style,

    /// Style for values next to labels.
    pub value: Style,

    /// Style for informational text (like keys).
    pub info: Style,

    /// Style for placeholder text.
    pub placeholder: Style,

    /// Style for "before" text in step details.
    pub text_before: Style,

    /// Style for "after" text in step details.
    pub text_after: Style,

    /// Style for description text.
    pub description: Style,

    /// Style for links.
    pub link: Style,
}

impl TuiColors {
    /// Creates a [`TuiColors`] instance from the given configuration.
    ///
    /// This method extracts colors from the config's colorscheme hashmap
    /// and converts them to Ratatui styles. If a color key is missing or
    /// cannot be parsed, sensible defaults are used.
    pub fn from_config(config: &Config) -> Self {
        let informational = config
            .colourscheme
            .get("informational")
            .and_then(|s| parse_color_string(s))
            .unwrap_or(Color::Rgb(255, 215, 0)); // Gold yellow fallback

        let success_color = config
            .colourscheme
            .get("success")
            .and_then(|s| parse_color_string(s))
            .unwrap_or(Color::Rgb(0, 255, 0)); // Green fallback

        let warning = config
            .colourscheme
            .get("warning")
            .and_then(|s| parse_color_string(s))
            .unwrap_or(Color::Rgb(255, 165, 0)); // Orange fallback

        let error = config
            .colourscheme
            .get("error")
            .and_then(|s| parse_color_string(s))
            .unwrap_or(Color::Rgb(255, 0, 0)); // Red fallback

        Self {
            primary: Style::default().fg(informational),
            success: Style::default().fg(success_color),
            warning: Style::default().fg(warning),
            error: Style::default().fg(error),
            text: Style::default().fg(Color::White),
            text_dimmed: Style::default().fg(Color::DarkGray),
            muted: Style::default().fg(Color::DarkGray),
            highlight: Style::default()
                .fg(informational)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Gray),
            accent: Style::default().fg(informational),
            title: Style::default()
                .fg(informational)
                .add_modifier(Modifier::BOLD),
            label: Style::default().fg(Color::Cyan),
            value: Style::default().fg(Color::White),
            info: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
            placeholder: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
            text_before: Style::default().fg(Color::Yellow),
            text_after: Style::default().fg(success_color),
            description: Style::default().fg(Color::Gray),
            link: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
        }
    }
}

impl Default for TuiColors {
    /// Creates a [`TuiColors`] instance with default colors.
    ///
    /// This is useful when the configuration is not available or as a fallback.
    /// The default colors match Ciphey's default colorscheme.
    fn default() -> Self {
        let gold = Color::Rgb(255, 215, 0);
        let green = Color::Rgb(0, 255, 0);

        Self {
            primary: Style::default().fg(gold),
            success: Style::default().fg(green),
            warning: Style::default().fg(Color::Rgb(255, 165, 0)),
            error: Style::default().fg(Color::Rgb(255, 0, 0)),
            text: Style::default().fg(Color::White),
            text_dimmed: Style::default().fg(Color::DarkGray),
            muted: Style::default().fg(Color::DarkGray),
            highlight: Style::default().fg(gold).add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Gray),
            accent: Style::default().fg(gold),
            title: Style::default().fg(gold).add_modifier(Modifier::BOLD),
            label: Style::default().fg(Color::Cyan),
            value: Style::default().fg(Color::White),
            info: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
            placeholder: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
            text_before: Style::default().fg(Color::Yellow),
            text_after: Style::default().fg(green),
            description: Style::default().fg(Color::Gray),
            link: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
        }
    }
}

/// Parses a color string into a Ratatui [`Color`].
///
/// Supports two formats:
/// - RGB comma-separated: `"255,215,0"`
/// - Hex format: `"#ffd700"` or `"ffd700"`
pub fn parse_color_string(color_str: &str) -> Option<Color> {
    let trimmed = color_str.trim();

    // Try hex format first (starts with # or looks like hex)
    if trimmed.starts_with('#') {
        return hex_to_color(trimmed);
    }

    // Try RGB comma-separated format: "255,215,0"
    if trimmed.contains(',') {
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() == 3 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            return Some(Color::Rgb(r, g, b));
        }
    }

    // Try hex without # prefix (6 hex chars)
    if trimmed.len() == 6 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return hex_to_color(&format!("#{}", trimmed));
    }

    None
}

/// Parses a hex color string into a Ratatui [`Color`].
///
/// Supports the following formats:
/// - `#RRGGBB` (e.g., `#ff0000` for red)
/// - `#RGB` shorthand (e.g., `#f00` expands to `#ff0000`)
pub fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim().trim_start_matches('#');

    match hex.len() {
        // Short format: #RGB -> #RRGGBB
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Color::Rgb(r, g, b))
        }
        // Full format: #RRGGBB
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::Rgb(r, g, b))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_color_full_format() {
        assert_eq!(hex_to_color("#ff0000"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(hex_to_color("#00ff00"), Some(Color::Rgb(0, 255, 0)));
        assert_eq!(hex_to_color("#0000ff"), Some(Color::Rgb(0, 0, 255)));
    }

    #[test]
    fn test_hex_to_color_short_format() {
        assert_eq!(hex_to_color("#f00"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(hex_to_color("#0f0"), Some(Color::Rgb(0, 255, 0)));
    }

    #[test]
    fn test_parse_color_string_rgb_format() {
        assert_eq!(parse_color_string("255,0,0"), Some(Color::Rgb(255, 0, 0)));
        assert_eq!(
            parse_color_string("255, 215, 0"),
            Some(Color::Rgb(255, 215, 0))
        );
    }

    #[test]
    fn test_tui_colors_default() {
        let colors = TuiColors::default();
        assert_eq!(colors.primary.fg, Some(Color::Rgb(255, 215, 0)));
        assert_eq!(colors.success.fg, Some(Color::Rgb(0, 255, 0)));
    }
}
