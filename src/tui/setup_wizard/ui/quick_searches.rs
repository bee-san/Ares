//! Quick Searches configuration screen rendering for the setup wizard.
//!
//! Allows users to configure URL templates for the "Open" shortcut (`o` key)
//! in the TUI Results screen. Each entry maps a name to a URL template
//! where `{}` is replaced with the decoded output text.

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};

use super::tutorial::centered_rect;

/// Primary accent color (gold)
const ACCENT: Color = Color::Rgb(255, 215, 0);
/// Muted text color
const MUTED: Color = Color::DarkGray;
/// Secondary accent
const SECONDARY: Color = Color::Rgb(139, 233, 253);

/// Draws the quick searches configuration screen.
///
/// Shows a list of current quick search entries with the ability to add/remove.
///
/// # Arguments
///
/// * `frame` - The Ratatui frame to render into
/// * `area` - The area to render within
/// * `entries` - Current list of quick search entries
/// * `selected` - Currently selected entry index
/// * `current_input` - Text being typed for a new entry
/// * `cursor` - Cursor position in the input field
pub fn draw_quick_searches(
    frame: &mut Frame,
    area: Rect,
    entries: &[String],
    selected: usize,
    current_input: &str,
    _cursor: usize,
) {
    let content_area = centered_rect(area, 80, 90);

    let block = Block::default()
        .title(" Quick Searches ")
        .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    // Layout: description, entries list, input field
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Description
            Constraint::Min(5),    // Entries list
            Constraint::Length(3), // Input field
        ])
        .split(inner);

    // Description
    let desc = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::White)),
            Span::styled(
                "[o]",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " in the results screen to open decoded text in these services.",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Format: Name=https://example.com/search?q={} (use {base64} for base64 encoding)",
            Style::default().fg(MUTED),
        )),
    ])
    .wrap(Wrap { trim: false });
    frame.render_widget(desc, chunks[0]);

    // Entries list
    let mut entry_lines: Vec<Line> = Vec::new();

    if entries.is_empty() {
        entry_lines.push(Line::from(Span::styled(
            "  (no entries - add one below)",
            Style::default().fg(MUTED),
        )));
    } else {
        for (i, entry) in entries.iter().enumerate() {
            let is_selected = i == selected;
            // Parse name from "Name=URL"
            let name = entry.splitn(2, '=').next().unwrap_or(entry);
            let style = if is_selected {
                Style::default()
                    .fg(ACCENT)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if is_selected { "> " } else { "  " };
            entry_lines.push(Line::from(Span::styled(
                format!("{}{}", prefix, name),
                style,
            )));
        }
    }

    let entries_widget = Paragraph::new(entry_lines);
    frame.render_widget(entries_widget, chunks[1]);

    // Input field
    let input_block = Block::default()
        .title(" Add new (Enter to add, Enter on empty to continue) ")
        .title_style(Style::default().fg(SECONDARY))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let input_inner = input_block.inner(chunks[2]);
    frame.render_widget(input_block, chunks[2]);

    // Render input text with cursor
    let mut input_spans = vec![Span::styled(
        current_input,
        Style::default().fg(Color::White),
    )];
    input_spans.push(Span::styled("_", Style::default().fg(ACCENT)));

    let input_text = Paragraph::new(Line::from(input_spans));
    frame.render_widget(input_text, input_inner);
}
