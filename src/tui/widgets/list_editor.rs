//! List editor widget for editing string lists in settings.
//!
//! This module provides a modal dialog for editing lists of strings,
//! such as lemmeknow tags.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph};

use crate::tui::colors::TuiColors;

/// List editor widget for managing string lists.
#[derive(Debug)]
pub struct ListEditor;

impl ListEditor {
    /// Creates a new list editor widget.
    pub fn new() -> Self {
        Self
    }

    /// Renders the list editor modal.
    ///
    /// # Arguments
    ///
    /// * `area` - The full screen area (for modal centering)
    /// * `buf` - The buffer to render into
    /// * `field_label` - The name of the field being edited
    /// * `items` - The list of items
    /// * `selected_item` - Index of the selected item (None if adding new)
    /// * `input_buffer` - Current input for new/editing item
    /// * `cursor_pos` - Cursor position in input buffer
    /// * `colors` - The color scheme to use
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        field_label: &str,
        items: &[String],
        selected_item: Option<usize>,
        input_buffer: &str,
        _cursor_pos: usize,
        colors: &TuiColors,
    ) {
        // Calculate modal size (60% width, 70% height)
        let modal_width = area.width * 60 / 100;
        let modal_height = area.height * 70 / 100;
        let modal_x = area.x + (area.width - modal_width) / 2;
        let modal_y = area.y + (area.height - modal_height) / 2;

        let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

        // Clear the area behind the modal
        Clear.render(modal_area, buf);

        // Create modal block
        let title = format!(" Edit: {} ", field_label);
        let block = Block::default()
            .title(title)
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(colors.accent)
            .padding(Padding::horizontal(1));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        // Layout: items list, input field, instructions
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),    // Items list
                Constraint::Length(3), // Input field
                Constraint::Length(2), // Instructions
            ])
            .split(inner);

        // Render items list
        self.render_items_list(chunks[0], buf, items, selected_item, colors);

        // Render input field
        self.render_input_field(chunks[1], buf, input_buffer, colors);

        // Render instructions
        self.render_instructions(chunks[2], buf, selected_item, colors);
    }

    /// Renders the list of items with selection indicator.
    fn render_items_list(
        &self,
        area: Rect,
        buf: &mut Buffer,
        items: &[String],
        selected_item: Option<usize>,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Items ")
            .title_style(colors.label)
            .borders(Borders::ALL)
            .border_style(colors.border);

        let inner = block.inner(area);
        block.render(area, buf);

        if items.is_empty() {
            let empty_msg =
                Paragraph::new(Span::styled("No items. Type below to add.", colors.muted))
                    .alignment(Alignment::Center);
            empty_msg.render(inner, buf);
            return;
        }

        // Render each item
        for (i, item) in items.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }

            let y = inner.y + i as u16;
            let is_selected = selected_item == Some(i);

            let style = if is_selected {
                colors
                    .accent
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                colors.text
            };

            let prefix = if is_selected { "> " } else { "  " };
            let delete_hint = if is_selected { " [Del to remove]" } else { "" };
            let text = format!("{}{}{}", prefix, item, delete_hint);

            // Truncate if too long
            let max_len = inner.width as usize;
            let display_text = if text.len() > max_len {
                format!("{}...", &text[..max_len.saturating_sub(3)])
            } else {
                text
            };

            let span = Span::styled(display_text, style);
            buf.set_span(inner.x, y, &span, inner.width);
        }
    }

    /// Renders the input field for adding/editing items.
    fn render_input_field(
        &self,
        area: Rect,
        buf: &mut Buffer,
        input_buffer: &str,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" New Item ")
            .title_style(colors.label)
            .borders(Borders::ALL)
            .border_style(colors.border);

        let inner = block.inner(area);
        block.render(area, buf);

        // Show input with cursor
        let display_text = if input_buffer.is_empty() {
            "Type here to add new item...".to_string()
        } else {
            format!("{}_", input_buffer)
        };

        let style = if input_buffer.is_empty() {
            colors.placeholder
        } else {
            colors.highlight
        };

        let paragraph = Paragraph::new(Span::styled(display_text, style));
        paragraph.render(inner, buf);
    }

    /// Renders the instruction footer.
    fn render_instructions(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selected_item: Option<usize>,
        colors: &TuiColors,
    ) {
        let instructions = if selected_item.is_some() {
            vec![
                Span::styled("[Enter]", colors.accent),
                Span::styled(" Add  ", colors.muted),
                Span::styled("[Del/Backspace]", colors.accent),
                Span::styled(" Remove  ", colors.muted),
                Span::styled("[Up/Down]", colors.accent),
                Span::styled(" Select  ", colors.muted),
                Span::styled("[Esc]", colors.accent),
                Span::styled(" Done", colors.muted),
            ]
        } else {
            vec![
                Span::styled("[Enter]", colors.accent),
                Span::styled(" Add  ", colors.muted),
                Span::styled("[Up/Down]", colors.accent),
                Span::styled(" Select  ", colors.muted),
                Span::styled("[Esc]", colors.accent),
                Span::styled(" Done", colors.muted),
            ]
        };

        let paragraph = Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);
        paragraph.render(area, buf);
    }
}

impl Default for ListEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Renders the list editor modal centered on the screen.
#[allow(clippy::too_many_arguments)]
pub fn render_list_editor(
    area: Rect,
    buf: &mut Buffer,
    field_label: &str,
    items: &[String],
    selected_item: Option<usize>,
    input_buffer: &str,
    cursor_pos: usize,
    colors: &TuiColors,
) {
    let editor = ListEditor::new();
    editor.render(
        area,
        buf,
        field_label,
        items,
        selected_item,
        input_buffer,
        cursor_pos,
        colors,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_editor_creation() {
        let editor = ListEditor::new();
        assert!(std::mem::size_of_val(&editor) >= 0);
    }

    #[test]
    fn test_list_editor_default() {
        let editor = ListEditor::default();
        assert!(std::mem::size_of_val(&editor) >= 0);
    }
}
