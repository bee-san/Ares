//! Wordlist manager widget for the TUI.
//!
//! This module provides a table-based interface for managing wordlist files,
//! including enabling/disabling wordlists and adding new wordlist files.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Clear, Padding, Paragraph, Row, Table};

use crate::tui::app::WordlistFileInfo;
use crate::tui::colors::TuiColors;

/// Focus state for the wordlist manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordlistFocus {
    /// Navigating the wordlist table.
    Table,
    /// Entering a file path to add.
    AddPath,
    /// Focused on the Done button.
    Done,
}

/// Wordlist manager widget for managing wordlist files.
#[derive(Debug)]
pub struct WordlistManagerWidget;

impl WordlistManagerWidget {
    /// Creates a new wordlist manager widget.
    pub fn new() -> Self {
        Self
    }

    /// Renders the wordlist manager modal.
    ///
    /// # Arguments
    ///
    /// * `area` - The full screen area (for modal centering)
    /// * `buf` - The buffer to render into
    /// * `wordlist_files` - List of wordlist files to display
    /// * `selected_row` - Currently selected row index
    /// * `focus` - Current focus state
    /// * `path_input` - Input buffer for new wordlist path
    /// * `has_pending_changes` - Whether there are unsaved changes
    /// * `colors` - The color scheme to use
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        wordlist_files: &[WordlistFileInfo],
        selected_row: usize,
        focus: WordlistFocus,
        path_input: &str,
        has_pending_changes: bool,
        colors: &TuiColors,
    ) {
        // Calculate modal size (80% width, 80% height)
        let modal_width = area.width * 80 / 100;
        let modal_height = area.height * 80 / 100;
        let modal_x = area.x + (area.width - modal_width) / 2;
        let modal_y = area.y + (area.height - modal_height) / 2;

        let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

        // Clear the area behind the modal
        Clear.render(modal_area, buf);

        // Create modal block
        let title = if has_pending_changes {
            " Wordlist Manager (modified) "
        } else {
            " Wordlist Manager "
        };

        let block = Block::default()
            .title(title)
            .title_style(colors.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(colors.accent)
            .padding(Padding::new(1, 1, 0, 0));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        // Layout: table, add path input, buttons, instructions
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(8),    // Table
                Constraint::Length(3), // Add path input
                Constraint::Length(3), // Buttons
                Constraint::Length(2), // Instructions
            ])
            .split(inner);

        // Render table
        self.render_table(chunks[0], buf, wordlist_files, selected_row, focus, colors);

        // Render add path input
        self.render_add_path_input(chunks[1], buf, path_input, focus, colors);

        // Render buttons
        self.render_buttons(chunks[2], buf, focus, has_pending_changes, colors);

        // Render instructions
        self.render_instructions(chunks[3], buf, focus, colors);
    }

    /// Renders the wordlist files table.
    fn render_table(
        &self,
        area: Rect,
        buf: &mut Buffer,
        wordlist_files: &[WordlistFileInfo],
        selected_row: usize,
        focus: WordlistFocus,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Wordlists ")
            .title_style(colors.label)
            .borders(Borders::ALL)
            .border_style(if focus == WordlistFocus::Table {
                colors.accent
            } else {
                colors.border
            });

        let inner = block.inner(area);
        block.render(area, buf);

        if wordlist_files.is_empty() {
            let empty_msg = Paragraph::new(Span::styled(
                "No wordlists found. Add a wordlist file below.",
                colors.muted,
            ))
            .alignment(Alignment::Center);
            empty_msg.render(inner, buf);
            return;
        }

        // Create table headers
        let header = Row::new(vec![
            Cell::from("Enabled").style(colors.label.add_modifier(Modifier::BOLD)),
            Cell::from("Filename").style(colors.label.add_modifier(Modifier::BOLD)),
            Cell::from("Words").style(colors.label.add_modifier(Modifier::BOLD)),
            Cell::from("Source").style(colors.label.add_modifier(Modifier::BOLD)),
            Cell::from("Added").style(colors.label.add_modifier(Modifier::BOLD)),
        ])
        .height(1);

        // Create table rows
        let rows: Vec<Row> = wordlist_files
            .iter()
            .enumerate()
            .map(|(i, wl)| {
                let is_selected = i == selected_row && focus == WordlistFocus::Table;
                let enabled_text = if wl.enabled { "[x]" } else { "[ ]" };

                let row_style = if is_selected {
                    colors.accent.add_modifier(Modifier::REVERSED)
                } else if !wl.enabled {
                    colors.muted
                } else {
                    colors.text
                };

                Row::new(vec![
                    Cell::from(enabled_text),
                    Cell::from(truncate_string(&wl.filename, 25)),
                    Cell::from(format!("{}", wl.word_count)),
                    Cell::from(truncate_string(&wl.source, 15)),
                    Cell::from(truncate_string(&wl.added_date, 10)),
                ])
                .style(row_style)
            })
            .collect();

        // Calculate column widths
        let widths = [
            Constraint::Length(8),  // Enabled
            Constraint::Min(20),    // Filename
            Constraint::Length(10), // Words
            Constraint::Length(15), // Source
            Constraint::Length(12), // Added
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(colors.accent.add_modifier(Modifier::REVERSED));

        // Manually render table without StatefulWidget (which requires mutable state)
        ratatui::widgets::Widget::render(table, inner, buf);
    }

    /// Renders the add path input field.
    fn render_add_path_input(
        &self,
        area: Rect,
        buf: &mut Buffer,
        path_input: &str,
        focus: WordlistFocus,
        colors: &TuiColors,
    ) {
        let block = Block::default()
            .title(" Add Wordlist Path ")
            .title_style(colors.label)
            .borders(Borders::ALL)
            .border_style(if focus == WordlistFocus::AddPath {
                colors.accent
            } else {
                colors.border
            });

        let inner = block.inner(area);
        block.render(area, buf);

        let display_text = if path_input.is_empty() {
            if focus == WordlistFocus::AddPath {
                "Enter path to wordlist file..._".to_string()
            } else {
                "Enter path to wordlist file...".to_string()
            }
        } else {
            format!("{}_", path_input)
        };

        let style = if focus == WordlistFocus::AddPath {
            if path_input.is_empty() {
                colors.placeholder
            } else {
                colors.highlight
            }
        } else {
            colors.muted
        };

        let paragraph = Paragraph::new(Span::styled(display_text, style));
        paragraph.render(inner, buf);
    }

    /// Renders the action buttons.
    fn render_buttons(
        &self,
        area: Rect,
        buf: &mut Buffer,
        focus: WordlistFocus,
        has_pending_changes: bool,
        colors: &TuiColors,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Done button
        let done_style = if focus == WordlistFocus::Done {
            colors
                .success
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
        } else {
            colors.success
        };

        let done_text = if has_pending_changes {
            " [Save & Close] "
        } else {
            " [Done] "
        };

        let done_button =
            Paragraph::new(Span::styled(done_text, done_style)).alignment(Alignment::Center);
        done_button.render(chunks[0], buf);

        // Cancel hint
        let cancel_text = " [Esc to Cancel] ";
        let cancel_button =
            Paragraph::new(Span::styled(cancel_text, colors.muted)).alignment(Alignment::Center);
        cancel_button.render(chunks[1], buf);
    }

    /// Renders the instruction footer.
    fn render_instructions(
        &self,
        area: Rect,
        buf: &mut Buffer,
        focus: WordlistFocus,
        colors: &TuiColors,
    ) {
        let instructions = match focus {
            WordlistFocus::Table => vec![
                Span::styled("[Space]", colors.accent),
                Span::styled(" Toggle  ", colors.muted),
                Span::styled("[Del]", colors.accent),
                Span::styled(" Remove  ", colors.muted),
                Span::styled("[Tab]", colors.accent),
                Span::styled(" Next Field  ", colors.muted),
                Span::styled("[Enter]", colors.accent),
                Span::styled(" Done", colors.muted),
            ],
            WordlistFocus::AddPath => vec![
                Span::styled("[Enter]", colors.accent),
                Span::styled(" Add File  ", colors.muted),
                Span::styled("[Tab]", colors.accent),
                Span::styled(" Next Field  ", colors.muted),
                Span::styled("[Esc]", colors.accent),
                Span::styled(" Cancel", colors.muted),
            ],
            WordlistFocus::Done => vec![
                Span::styled("[Enter]", colors.accent),
                Span::styled(" Save & Close  ", colors.muted),
                Span::styled("[Tab]", colors.accent),
                Span::styled(" Back to Table  ", colors.muted),
                Span::styled("[Esc]", colors.accent),
                Span::styled(" Cancel", colors.muted),
            ],
        };

        let paragraph = Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);
        paragraph.render(area, buf);
    }
}

impl Default for WordlistManagerWidget {
    fn default() -> Self {
        Self::new()
    }
}

/// Truncates a string to the specified maximum length.
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Renders the wordlist manager modal.
#[allow(clippy::too_many_arguments)]
pub fn render_wordlist_manager(
    area: Rect,
    buf: &mut Buffer,
    wordlist_files: &[WordlistFileInfo],
    selected_row: usize,
    focus: WordlistFocus,
    path_input: &str,
    has_pending_changes: bool,
    colors: &TuiColors,
) {
    let widget = WordlistManagerWidget::new();
    widget.render(
        area,
        buf,
        wordlist_files,
        selected_row,
        focus,
        path_input,
        has_pending_changes,
        colors,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordlist_manager_creation() {
        let widget = WordlistManagerWidget::new();
        assert!(std::mem::size_of_val(&widget) >= 0);
    }

    #[test]
    fn test_truncate_string_short() {
        assert_eq!(truncate_string("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_string_long() {
        assert_eq!(truncate_string("hello world", 8), "hello...");
    }

    #[test]
    fn test_wordlist_focus_equality() {
        assert_eq!(WordlistFocus::Table, WordlistFocus::Table);
        assert_ne!(WordlistFocus::Table, WordlistFocus::AddPath);
    }
}
