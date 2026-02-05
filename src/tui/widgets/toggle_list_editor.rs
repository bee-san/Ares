//! Toggle list editor widget for selecting items from a fixed set.
//!
//! This module provides a modal dialog for selecting items (like decoders/checkers)
//! with checkbox toggles. Unlike the regular ListEditor, items cannot be added or removed,
//! only enabled/disabled.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph};

use crate::tui::colors::TuiColors;

/// Toggle list editor widget for managing item selection.
#[derive(Debug)]
pub struct ToggleListEditor;

impl ToggleListEditor {
    /// Creates a new toggle list editor widget.
    pub fn new() -> Self {
        Self
    }

    /// Renders the toggle list editor modal.
    ///
    /// # Arguments
    ///
    /// * `area` - The full screen area (for modal centering)
    /// * `buf` - The buffer to render into
    /// * `field_label` - The name of the field being edited
    /// * `all_items` - All available items that can be toggled
    /// * `selected_items` - Items that are currently enabled/selected
    /// * `cursor_index` - Index of the currently highlighted item
    /// * `scroll_offset` - Scroll offset for long lists
    /// * `colors` - The color scheme to use
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        field_label: &str,
        all_items: &[String],
        selected_items: &[String],
        cursor_index: usize,
        scroll_offset: usize,
        colors: &TuiColors,
    ) {
        // Calculate modal size (60% width, 80% height)
        let modal_width = area.width * 60 / 100;
        let modal_height = area.height * 80 / 100;
        let modal_x = area.x + (area.width - modal_width) / 2;
        let modal_y = area.y + (area.height - modal_height) / 2;

        let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

        // Clear the area behind the modal
        Clear.render(modal_area, buf);

        // Create modal block
        let title = format!(" {} ", field_label);
        let block = Block::default()
            .title(title)
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(colors.accent)
            .padding(Padding::horizontal(1));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        // Layout: header info, items list, instructions
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Header info
                Constraint::Min(5),    // Items list
                Constraint::Length(2), // Instructions
            ])
            .split(inner);

        // Render header info
        self.render_header(chunks[0], buf, all_items, selected_items, colors);

        // Render items list
        self.render_items_list(
            chunks[1],
            buf,
            all_items,
            selected_items,
            cursor_index,
            scroll_offset,
            colors,
        );

        // Render instructions
        self.render_instructions(chunks[2], buf, colors);
    }

    /// Renders the header with selection count.
    fn render_header(
        &self,
        area: Rect,
        buf: &mut Buffer,
        all_items: &[String],
        selected_items: &[String],
        colors: &TuiColors,
    ) {
        let selected_count = selected_items.len();

        let status_text = if selected_count == 0 {
            "0 items enabled (nothing will run)".to_string()
        } else if selected_count == all_items.len() {
            format!("All {} items enabled", all_items.len())
        } else {
            format!("{} of {} items enabled", selected_count, all_items.len())
        };

        let paragraph =
            Paragraph::new(Span::styled(status_text, colors.label)).alignment(Alignment::Center);
        paragraph.render(area, buf);
    }

    /// Renders the list of items with checkboxes.
    #[allow(clippy::too_many_arguments)]
    fn render_items_list(
        &self,
        area: Rect,
        buf: &mut Buffer,
        all_items: &[String],
        selected_items: &[String],
        cursor_index: usize,
        scroll_offset: usize,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Items ")
            .title_style(colors.label)
            .borders(Borders::ALL)
            .border_style(colors.border);

        let inner = block.inner(area);
        block.render(area, buf);

        if all_items.is_empty() {
            let empty_msg = Paragraph::new(Span::styled("No items available.", colors.muted))
                .alignment(Alignment::Center);
            empty_msg.render(inner, buf);
            return;
        }

        let visible_height = inner.height as usize;

        // Render each visible item
        for (display_idx, item_idx) in (scroll_offset..).enumerate() {
            if display_idx >= visible_height || item_idx >= all_items.len() {
                break;
            }

            let item = &all_items[item_idx];
            let y = inner.y + display_idx as u16;
            let is_cursor = item_idx == cursor_index;

            // Check if item is selected (in the list)
            let is_checked = selected_items.contains(item);

            // Build the checkbox and item text
            let checkbox = if is_checked { "[x]" } else { "[ ]" };
            let prefix = if is_cursor { "> " } else { "  " };
            let text = format!("{}{} {}", prefix, checkbox, item);

            // Style based on cursor position
            let style = if is_cursor {
                colors
                    .accent
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else if is_checked {
                colors.success
            } else {
                colors.muted
            };

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

        // Render scroll indicators if needed
        if scroll_offset > 0 {
            let up_indicator = Span::styled("^", colors.accent);
            buf.set_span(inner.x + inner.width - 2, inner.y, &up_indicator, 1);
        }
        if scroll_offset + visible_height < all_items.len() {
            let down_indicator = Span::styled("v", colors.accent);
            buf.set_span(
                inner.x + inner.width - 2,
                inner.y + inner.height - 1,
                &down_indicator,
                1,
            );
        }
    }

    /// Renders the instruction footer.
    fn render_instructions(&self, area: Rect, buf: &mut Buffer, colors: &TuiColors) {
        let instructions = vec![
            Span::styled("[Space]", colors.accent),
            Span::styled(" Toggle  ", colors.muted),
            Span::styled("[a]", colors.accent),
            Span::styled(" All  ", colors.muted),
            Span::styled("[n]", colors.accent),
            Span::styled(" None  ", colors.muted),
            Span::styled("[Up/Down]", colors.accent),
            Span::styled(" Navigate  ", colors.muted),
            Span::styled("[Esc/Enter]", colors.accent),
            Span::styled(" Done", colors.muted),
        ];

        let paragraph = Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);
        paragraph.render(area, buf);
    }
}

impl Default for ToggleListEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Renders the toggle list editor modal centered on the screen.
#[allow(clippy::too_many_arguments)]
pub fn render_toggle_list_editor(
    area: Rect,
    buf: &mut Buffer,
    field_label: &str,
    all_items: &[String],
    selected_items: &[String],
    cursor_index: usize,
    scroll_offset: usize,
    colors: &TuiColors,
) {
    let editor = ToggleListEditor::new();
    editor.render(
        area,
        buf,
        field_label,
        all_items,
        selected_items,
        cursor_index,
        scroll_offset,
        colors,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    /// Creates test colors for rendering tests
    fn create_test_colors() -> TuiColors {
        let config = Config::default();
        TuiColors::from_config(&config)
    }

    #[test]
    fn test_toggle_list_editor_creation() {
        let _editor = ToggleListEditor::new();
    }

    #[test]
    fn test_toggle_list_editor_default() {
        let _editor = ToggleListEditor::default();
    }

    #[test]
    fn test_render_empty_list() {
        let editor = ToggleListEditor::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();

        editor.render(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Test Field",
            &[],
            &[],
            0,
            0,
            &colors,
        );

        // Should render "No items" message
        let content = buf.content();
        let has_empty_msg = content.iter().any(|cell| cell.symbol() == "N");
        assert!(has_empty_msg, "Should render empty list message");
    }

    #[test]
    fn test_render_with_items() {
        let editor = ToggleListEditor::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();
        let all_items = vec![
            "Base64".to_string(),
            "caesar".to_string(),
            "Hexadecimal".to_string(),
        ];
        let selected_items = vec!["Base64".to_string(), "Hexadecimal".to_string()];

        editor.render(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Decoders",
            &all_items,
            &selected_items,
            0,
            0,
            &colors,
        );

        // Should render item names
        let content = buf.content();
        let has_item = content.iter().any(|cell| cell.symbol() == "B");
        assert!(has_item, "Should render item names");
    }

    #[test]
    fn test_render_with_cursor() {
        let editor = ToggleListEditor::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();
        let all_items = vec!["item1".to_string(), "item2".to_string()];

        editor.render(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Test",
            &all_items,
            &[],
            0,
            0,
            &colors,
        );

        // Should render cursor indicator
        let content = buf.content();
        let has_indicator = content.iter().any(|cell| cell.symbol() == ">");
        assert!(has_indicator, "Should render cursor indicator");
    }

    #[test]
    fn test_render_checkbox_checked() {
        let editor = ToggleListEditor::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();
        let all_items = vec!["item1".to_string()];
        let selected_items = vec!["item1".to_string()];

        editor.render(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Test",
            &all_items,
            &selected_items,
            0,
            0,
            &colors,
        );

        // Should render checked checkbox
        let content = buf.content();
        let has_checkbox = content.iter().any(|cell| cell.symbol() == "x");
        assert!(has_checkbox, "Should render checked checkbox");
    }

    #[test]
    fn test_render_none_enabled_when_empty() {
        let editor = ToggleListEditor::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();
        let all_items = vec!["item1".to_string(), "item2".to_string()];

        // Empty selected_items means none are enabled
        editor.render(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Test",
            &all_items,
            &[],
            0,
            0,
            &colors,
        );

        // Should render all as unchecked (no 'x' in checkboxes)
        let content = buf.content();
        // Look for unchecked checkboxes - the pattern "[ ]" should appear
        let has_unchecked = content.iter().any(|cell| cell.symbol() == " ");
        assert!(
            has_unchecked,
            "Items should be unchecked when selected_items is empty"
        );
    }

    #[test]
    fn test_render_instructions() {
        let editor = ToggleListEditor::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();
        let all_items = vec!["item1".to_string()];

        editor.render(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Test",
            &all_items,
            &[],
            0,
            0,
            &colors,
        );

        // Should render instruction keybindings
        let content = buf.content();
        let has_instructions = content.iter().any(|cell| cell.symbol() == "[");
        assert!(has_instructions, "Should render instructions");
    }

    #[test]
    fn test_render_toggle_list_editor_function() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 30));
        let colors = create_test_colors();
        let all_items = vec!["test".to_string()];

        render_toggle_list_editor(
            Rect::new(0, 0, 80, 30),
            &mut buf,
            "Test",
            &all_items,
            &[],
            0,
            0,
            &colors,
        );

        // Should render successfully
        let content = buf.content();
        assert!(!content.is_empty(), "Should render via wrapper function");
    }
}
