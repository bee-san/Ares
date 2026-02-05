//! Settings panel widget for the TUI.
//!
//! This module provides the settings form rendering component that displays
//! all configurable settings organized by section.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};

use crate::tui::colors::TuiColors;
use crate::tui::settings::{FieldType, SettingField, SettingValue, SettingsModel};

/// Settings panel widget for rendering the settings form.
#[derive(Debug)]
pub struct SettingsPanel;

impl SettingsPanel {
    /// Creates a new settings panel widget.
    pub fn new() -> Self {
        Self
    }

    /// Renders the settings panel.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render within
    /// * `buf` - The buffer to render into
    /// * `settings` - The settings model to display
    /// * `selected_section` - Index of the currently selected section
    /// * `selected_field` - Index of the currently selected field within the section
    /// * `editing_mode` - Whether we're currently editing a field value
    /// * `input_buffer` - The current input buffer (for editing mode)
    /// * `cursor_pos` - The cursor position in the input buffer
    /// * `validation_errors` - Map of field_id -> error message
    /// * `colors` - The color scheme to use
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        settings: &SettingsModel,
        selected_section: usize,
        selected_field: usize,
        editing_mode: bool,
        input_buffer: &str,
        cursor_pos: usize,
        scroll_offset: usize,
        validation_errors: &std::collections::HashMap<String, String>,
        has_changes: bool,
        colors: &TuiColors,
    ) {
        // Create main layout: sections list on left, fields on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(22), // Section list
                Constraint::Min(40),    // Field editor
            ])
            .split(area);

        // Render sections list
        self.render_sections_list(chunks[0], buf, settings, selected_section, colors);

        // Render fields for selected section
        self.render_fields(
            chunks[1],
            buf,
            settings,
            selected_section,
            selected_field,
            editing_mode,
            input_buffer,
            cursor_pos,
            scroll_offset,
            validation_errors,
            has_changes,
            colors,
        );
    }

    /// Renders the sections list on the left side.
    fn render_sections_list(
        &self,
        area: Rect,
        buf: &mut Buffer,
        settings: &SettingsModel,
        selected_section: usize,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Sections ")
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_style(colors.border);

        let inner = block.inner(area);
        block.render(area, buf);

        // Render each section name
        for (i, section) in settings.sections.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }

            let y = inner.y + i as u16;
            let is_selected = i == selected_section;

            let style = if is_selected {
                colors
                    .accent
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                colors.text
            };

            let prefix = if is_selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, section.name);
            let span = Span::styled(text, style);

            buf.set_span(inner.x, y, &span, inner.width);
        }
    }

    /// Renders the fields for the selected section.
    #[allow(clippy::too_many_arguments)]
    fn render_fields(
        &self,
        area: Rect,
        buf: &mut Buffer,
        settings: &SettingsModel,
        selected_section: usize,
        selected_field: usize,
        editing_mode: bool,
        input_buffer: &str,
        _cursor_pos: usize,
        scroll_offset: usize,
        validation_errors: &std::collections::HashMap<String, String>,
        has_changes: bool,
        colors: &TuiColors,
    ) {
        let section = match settings.sections.get(selected_section) {
            Some(s) => s,
            None => return,
        };

        // Create block with section title
        let title = format!(" {} ", section.name);
        let block = Block::default()
            .title(title)
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_style(colors.border)
            .padding(Padding::horizontal(1));

        let inner = block.inner(area);
        block.render(area, buf);

        // Calculate total content height for scrolling
        let total_fields = section.fields.len();
        let lines_per_field = 3; // Label + value + spacing
        let _total_height = total_fields * lines_per_field;

        // Build lines for all fields
        let mut lines: Vec<Line> = Vec::new();

        for (i, field) in section.fields.iter().enumerate() {
            let is_selected = i == selected_field;
            let has_error = validation_errors.contains_key(field.id);
            let is_modified = field.value != field.original_value;

            // Field label with indicator
            let label_prefix = if is_selected { "> " } else { "  " };
            let modified_indicator = if is_modified { " *" } else { "" };
            let label_text = format!("{}{}{}", label_prefix, field.label, modified_indicator);

            let label_style = if is_selected {
                if has_error {
                    colors.error.add_modifier(Modifier::BOLD)
                } else {
                    colors.accent.add_modifier(Modifier::BOLD)
                }
            } else if has_error {
                colors.error
            } else {
                colors.label
            };

            lines.push(Line::from(Span::styled(label_text, label_style)));

            // Field value or input buffer
            let value_text = if is_selected && editing_mode {
                format!("    {} _", input_buffer)
            } else {
                format!("    {}", self.format_field_value(field))
            };

            let value_style = if is_selected && editing_mode {
                colors.highlight
            } else if field.opens_modal() {
                colors.info
            } else {
                colors.text
            };

            lines.push(Line::from(Span::styled(value_text, value_style)));

            // Show error message if present
            if has_error {
                if let Some(error_msg) = validation_errors.get(field.id) {
                    lines.push(Line::from(Span::styled(
                        format!("    Error: {}", error_msg),
                        colors.error,
                    )));
                }
            }

            // Description (dimmed)
            lines.push(Line::from(Span::styled(
                format!("    {}", field.description),
                colors.muted,
            )));

            // Empty line between fields
            lines.push(Line::from(""));
        }

        // Add status bar info at bottom
        lines.push(Line::from(""));
        let status_text = if editing_mode {
            "[Enter] Save  [Esc] Cancel"
        } else {
            "[Ctrl+S] Save  [Esc] Close  [Enter] Edit"
        };
        lines.push(Line::from(Span::styled(status_text, colors.muted)));

        // Apply scroll offset
        let visible_lines: Vec<Line> = lines
            .into_iter()
            .skip(scroll_offset)
            .take(inner.height as usize)
            .collect();

        let paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });
        paragraph.render(inner, buf);
    }

    /// Formats a field value for display.
    fn format_field_value(&self, field: &SettingField) -> String {
        match &field.field_type {
            FieldType::Boolean => {
                if let SettingValue::Bool(v) = &field.value {
                    if *v { "[x] Enabled" } else { "[ ] Disabled" }.to_string()
                } else {
                    field.display_value()
                }
            }
            FieldType::RgbColor => {
                if let SettingValue::Text(v) = &field.value {
                    format!("{} (preview)", v)
                } else {
                    field.display_value()
                }
            }
            FieldType::StringList => {
                if let SettingValue::List(v) = &field.value {
                    if v.is_empty() {
                        "(empty) [Press Enter to edit]".to_string()
                    } else if v.len() <= 2 {
                        format!("{} [Press Enter to edit]", v.join(", "))
                    } else {
                        format!("{} items [Press Enter to edit]", v.len())
                    }
                } else {
                    field.display_value()
                }
            }
            FieldType::WordlistManager => "[Press Enter to manage wordlists]".to_string(),
            FieldType::ThemePicker => "[Press Enter to choose theme]".to_string(),
            _ => field.display_value(),
        }
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Renders the settings screen including header and footer.
pub fn render_settings_screen(
    area: Rect,
    buf: &mut Buffer,
    settings: &SettingsModel,
    selected_section: usize,
    selected_field: usize,
    editing_mode: bool,
    input_buffer: &str,
    cursor_pos: usize,
    scroll_offset: usize,
    validation_errors: &std::collections::HashMap<String, String>,
    has_changes: bool,
    colors: &TuiColors,
) {
    // Create outer block with title
    let title = if has_changes {
        " Settings (modified) "
    } else {
        " Settings "
    };

    let outer_block = Block::default()
        .title(title)
        .title_style(colors.title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(colors.accent);

    let inner = outer_block.inner(area);
    outer_block.render(area, buf);

    // Create layout: main content + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Settings panel
            Constraint::Length(1), // Footer
        ])
        .split(inner);

    // Render settings panel
    let panel = SettingsPanel::new();
    panel.render(
        chunks[0],
        buf,
        settings,
        selected_section,
        selected_field,
        editing_mode,
        input_buffer,
        cursor_pos,
        scroll_offset,
        validation_errors,
        has_changes,
        colors,
    );

    // Render footer with keybinding hints
    let footer_spans = vec![
        Span::styled("[Tab]", colors.accent),
        Span::styled(" Section  ", colors.muted),
        Span::styled("[Up/Down]", colors.accent),
        Span::styled(" Navigate  ", colors.muted),
        Span::styled("[Enter]", colors.accent),
        Span::styled(" Edit  ", colors.muted),
        Span::styled("[Esc]", colors.accent),
        Span::styled(" Close", colors.muted),
    ];

    let footer = Paragraph::new(Line::from(footer_spans));
    footer.render(chunks[1], buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_panel_creation() {
        let panel = SettingsPanel::new();
        assert!(std::mem::size_of_val(&panel) >= 0);
    }

    #[test]
    fn test_format_boolean_value() {
        let panel = SettingsPanel::new();
        let field = SettingField {
            id: "test",
            label: "Test",
            description: "Test field",
            field_type: FieldType::Boolean,
            value: SettingValue::Bool(true),
            original_value: SettingValue::Bool(true),
        };

        let formatted = panel.format_field_value(&field);
        assert!(formatted.contains("Enabled"));
    }

    #[test]
    fn test_format_list_value() {
        let panel = SettingsPanel::new();
        let field = SettingField {
            id: "test",
            label: "Test",
            description: "Test field",
            field_type: FieldType::StringList,
            value: SettingValue::List(vec!["a".to_string(), "b".to_string()]),
            original_value: SettingValue::List(vec![]),
        };

        let formatted = panel.format_field_value(&field);
        assert!(formatted.contains("a, b"));
        assert!(formatted.contains("[Press Enter to edit]"));
    }
}
