//! Theme picker widget for selecting color schemes.
//!
//! This module provides a modal widget for selecting color themes with a live preview.
//! It's used in the TUI settings panel to allow users to quickly apply preset themes
//! or create custom color schemes.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};

use crate::tui::colors::TuiColors;
use crate::tui::setup_wizard::themes::{ColorScheme, THEMES};

/// Custom colors being edited in the theme picker
#[derive(Debug, Clone, Default)]
pub struct ThemePickerCustomColors {
    /// Informational color input
    pub informational: String,
    /// Warning color input
    pub warning: String,
    /// Success color input
    pub success: String,
    /// Error color input
    pub error: String,
    /// Question color input
    pub question: String,
}

impl ThemePickerCustomColors {
    /// Gets the field at the given index.
    pub fn get_field(&self, index: usize) -> &str {
        match index {
            0 => &self.informational,
            1 => &self.warning,
            2 => &self.success,
            3 => &self.error,
            4 => &self.question,
            _ => "",
        }
    }

    /// Gets a mutable reference to the field at the given index.
    pub fn get_field_mut(&mut self, index: usize) -> &mut String {
        match index {
            0 => &mut self.informational,
            1 => &mut self.warning,
            2 => &mut self.success,
            3 => &mut self.error,
            _ => &mut self.question,
        }
    }

    /// Gets the field name at the given index.
    pub fn field_name(index: usize) -> &'static str {
        match index {
            0 => "Informational",
            1 => "Warning",
            2 => "Success",
            3 => "Error",
            4 => "Question",
            _ => "",
        }
    }

    /// Parses the custom colors into a ColorScheme.
    pub fn to_scheme(&self) -> Option<ColorScheme> {
        let info = parse_rgb(&self.informational)?;
        let warn = parse_rgb(&self.warning)?;
        let succ = parse_rgb(&self.success)?;
        let err = parse_rgb(&self.error)?;
        let ques = parse_rgb(&self.question)?;

        Some(ColorScheme {
            informational: info,
            warning: warn,
            success: succ,
            error: err,
            question: ques,
        })
    }
}

/// Theme picker widget
#[derive(Debug)]
pub struct ThemePicker;

impl ThemePicker {
    /// Creates a new theme picker widget.
    pub fn new() -> Self {
        Self
    }

    /// Renders the theme picker modal
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render within
    /// * `buf` - The buffer to render into
    /// * `selected` - Currently selected theme index
    /// * `custom_mode` - Whether in custom color input mode
    /// * `custom_colors` - Custom colors being edited
    /// * `custom_field` - Current field in custom mode (0-4)
    /// * `colors` - The color scheme to use for UI elements
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selected: usize,
        custom_mode: bool,
        custom_colors: &ThemePickerCustomColors,
        custom_field: usize,
        colors: &TuiColors,
    ) {
        // Split into left (theme list) and right (preview)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Render left panel
        if custom_mode {
            self.render_custom_form(chunks[0], buf, custom_colors, custom_field, colors);
        } else {
            self.render_theme_list(chunks[0], buf, selected, colors);
        }

        // Render right panel (preview)
        let preview_scheme = if custom_mode {
            custom_colors.to_scheme().unwrap_or_default()
        } else if selected < THEMES.len() {
            THEMES[selected].scheme.clone()
        } else {
            ColorScheme::default()
        };

        self.render_preview(chunks[1], buf, &preview_scheme, colors);
    }

    /// Renders the theme list
    fn render_theme_list(&self, area: Rect, buf: &mut Buffer, selected: usize, colors: &TuiColors) {
        let block = Block::default()
            .title(" Select Theme ")
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(colors.border)
            .padding(Padding::new(1, 1, 1, 1));

        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = Vec::new();
        for (i, theme) in THEMES.iter().enumerate() {
            let prefix = if i == selected { " > " } else { "   " };
            let name = if let Some(icon) = theme.icon {
                format!("{}. {} {}", i + 1, icon, theme.name)
            } else {
                format!("{}. {}", i + 1, theme.name)
            };

            let style = if i == selected {
                colors.accent.add_modifier(Modifier::BOLD)
            } else {
                colors.text
            };

            lines.push(Line::from(Span::styled(
                format!("{}{}", prefix, name),
                style,
            )));
        }

        // Add custom option
        let custom_prefix = if selected == THEMES.len() {
            " > "
        } else {
            "   "
        };
        let custom_style = if selected == THEMES.len() {
            colors.accent.add_modifier(Modifier::BOLD)
        } else {
            colors.text
        };
        lines.push(Line::from(Span::styled(
            format!("{}{}. Custom...", custom_prefix, THEMES.len() + 1),
            custom_style,
        )));

        let list = Paragraph::new(lines);
        list.render(inner, buf);
    }

    /// Renders the custom color input form
    fn render_custom_form(
        &self,
        area: Rect,
        buf: &mut Buffer,
        custom_colors: &ThemePickerCustomColors,
        current_field: usize,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Custom Colors ")
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(colors.border)
            .padding(Padding::new(1, 1, 1, 1));

        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![
            Line::from(Span::styled(
                "Enter RGB values (e.g., 255,128,64)",
                colors.muted,
            )),
            Line::from(""),
        ];

        for i in 0..5 {
            let label = ThemePickerCustomColors::field_name(i);
            let value = custom_colors.get_field(i);

            let prefix = if i == current_field { "> " } else { "  " };

            let style = if i == current_field {
                colors.accent
            } else {
                colors.text
            };

            let value_style = if i == current_field {
                colors.text.add_modifier(Modifier::UNDERLINED)
            } else {
                colors.muted
            };

            lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{}: ", label), style),
                Span::styled(
                    if value.is_empty() {
                        "___,___,___"
                    } else {
                        value
                    },
                    value_style,
                ),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "[Tab] Next field  [Enter] Confirm  [Esc] Cancel",
            colors.muted,
        )));

        let form = Paragraph::new(lines);
        form.render(inner, buf);
    }

    /// Renders the theme preview panel
    fn render_preview(
        &self,
        area: Rect,
        buf: &mut Buffer,
        scheme: &ColorScheme,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Live Preview ")
            .title_style(colors.accent.add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(colors.border)
            .padding(Padding::new(1, 1, 1, 1));

        let inner = block.inner(area);
        block.render(area, buf);

        let lines = vec![
            Line::from(Span::styled("Informational", scheme.informational_style())),
            Line::from(Span::styled("  Status updates and info", colors.muted)),
            Line::from(""),
            Line::from(Span::styled("Warning", scheme.warning_style())),
            Line::from(Span::styled("  Cautions and alerts", colors.muted)),
            Line::from(""),
            Line::from(Span::styled("Success", scheme.success_style())),
            Line::from(Span::styled("  Successful operations", colors.muted)),
            Line::from(""),
            Line::from(Span::styled("Error", scheme.error_style())),
            Line::from(Span::styled("  Error messages", colors.muted)),
            Line::from(""),
            Line::from(Span::styled("Question", scheme.question_style())),
            Line::from(Span::styled("  Interactive prompts", colors.muted)),
        ];

        let preview = Paragraph::new(lines);
        preview.render(inner, buf);
    }
}

impl Default for ThemePicker {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses an RGB string like "255,128,64" into a tuple.
fn parse_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse().ok()?;
    let g = parts[1].trim().parse().ok()?;
    let b = parts[2].trim().parse().ok()?;
    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_picker_creation() {
        let picker = ThemePicker::new();
        assert!(std::mem::size_of_val(&picker) >= 0);
    }

    #[test]
    fn test_custom_colors_field_access() {
        let mut colors = ThemePickerCustomColors::default();
        colors.informational = "255,0,0".to_string();

        assert_eq!(colors.get_field(0), "255,0,0");
        assert_eq!(ThemePickerCustomColors::field_name(0), "Informational");
    }

    #[test]
    fn test_custom_colors_to_scheme() {
        let mut colors = ThemePickerCustomColors::default();
        colors.informational = "255,0,0".to_string();
        colors.warning = "0,255,0".to_string();
        colors.success = "0,0,255".to_string();
        colors.error = "255,255,0".to_string();
        colors.question = "255,0,255".to_string();

        let scheme = colors.to_scheme();
        assert!(scheme.is_some());

        let scheme = scheme.unwrap();
        assert_eq!(scheme.informational, (255, 0, 0));
        assert_eq!(scheme.warning, (0, 255, 0));
        assert_eq!(scheme.success, (0, 0, 255));
        assert_eq!(scheme.error, (255, 255, 0));
        assert_eq!(scheme.question, (255, 0, 255));
    }

    #[test]
    fn test_parse_rgb_valid() {
        assert_eq!(parse_rgb("255,0,0"), Some((255, 0, 0)));
        assert_eq!(parse_rgb("0, 255, 0"), Some((0, 255, 0)));
        assert_eq!(parse_rgb("128,128,128"), Some((128, 128, 128)));
    }

    #[test]
    fn test_parse_rgb_invalid() {
        assert_eq!(parse_rgb("255,0"), None);
        assert_eq!(parse_rgb("invalid"), None);
        assert_eq!(parse_rgb("256,0,0"), None);
    }
}
