//! Step details widget for displaying decoder information.
//!
//! This module provides a widget for rendering detailed information about
//! a selected decoder step in the TUI, including the decoder name, key,
//! before/after text, description, and reference link.

use ratatui::prelude::*;
use ratatui::style::Modifier;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::super::colors::TuiColors;
use crate::decoders::crack_results::CrackResult;

/// Maximum length for text before truncation.
const MAX_TEXT_LENGTH: usize = 200;

/// Truncates text to a maximum length, adding ellipsis if truncated.
///
/// # Arguments
///
/// * `text` - The text to potentially truncate
/// * `max_len` - Maximum allowed length before truncation
///
/// # Returns
///
/// The original text if within limit, or truncated text with "..." appended.
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

/// Renders detailed information about a decoder step.
///
/// Displays a box containing:
/// - Decoder name
/// - Key used (or "N/A" if none)
/// - Text before decoding (truncated if very long)
/// - Text after decoding (truncated if very long)
/// - Decoder description (word wrapped)
/// - Reference link
///
/// If no step is provided, displays a placeholder message.
///
/// # Arguments
///
/// * `area` - The rectangular area to render within
/// * `buf` - The buffer to render to
/// * `step` - Optional reference to the `CrackResult` to display
/// * `colors` - The TUI color scheme to use
///
/// # Example
///
/// ```ignore
/// use ratatui::prelude::*;
/// use ciphey::tui::widgets::step_details::render_step_details;
///
/// let area = Rect::new(0, 0, 80, 20);
/// let mut buf = Buffer::empty(area);
/// render_step_details(area, &mut buf, Some(&crack_result), &colors);
/// ```
pub fn render_step_details(
    area: Rect,
    buf: &mut Buffer,
    step: Option<&CrackResult>,
    colors: &TuiColors,
) {
    let block = Block::default()
        .title(" Step Details ")
        .borders(Borders::ALL)
        .border_style(colors.border)
        .title_style(colors.title);

    let inner_area = block.inner(area);
    block.render(area, buf);

    match step {
        Some(result) => {
            render_step_content(inner_area, buf, result, colors);
        }
        None => {
            render_placeholder(inner_area, buf, colors);
        }
    }
}

/// Renders the content of a decoder step.
///
/// # Arguments
///
/// * `area` - The inner area to render content within
/// * `buf` - The buffer to render to
/// * `result` - The `CrackResult` containing step information
/// * `colors` - The TUI color scheme to use
fn render_step_content(area: Rect, buf: &mut Buffer, result: &CrackResult, colors: &TuiColors) {
    let key_display = result.key.as_ref().map_or("N/A".to_string(), |k| k.clone());

    let before_text = truncate_text(&result.encrypted_text, MAX_TEXT_LENGTH);

    let after_text = result
        .unencrypted_text
        .as_ref()
        .map_or("N/A".to_string(), |texts| {
            if texts.is_empty() {
                "N/A".to_string()
            } else {
                truncate_text(&texts.join(", "), MAX_TEXT_LENGTH)
            }
        });

    // Build the content lines
    let mut lines = vec![
        Line::from(vec![
            Span::styled("Decoder: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(result.decoder, colors.value),
        ]),
        Line::from(vec![
            Span::styled("Key: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(key_display, colors.value),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Before: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(before_text, colors.text_before),
        ]),
        Line::from(vec![
            Span::styled("After:  ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(after_text, colors.text_after),
        ]),
        Line::from(""),
    ];

    // Add description with label
    if !result.description.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Description: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(result.description, colors.description),
        ]));
        lines.push(Line::from(""));
    }

    // Add link if present
    if !result.link.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Link: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(result.link, colors.link),
        ]));
        lines.push(Line::from(""));
    }

    // Add checker information if present
    if !result.checker_name.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Checker: ", colors.label.add_modifier(Modifier::BOLD)),
            Span::styled(result.checker_name, colors.checker_name),
        ]));

        // Add checker's extra info (e.g., what LemmeKnow identified)
        if !result.check_description.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Identified: ", colors.label.add_modifier(Modifier::BOLD)),
                Span::styled(&result.check_description, colors.checker_info),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

    paragraph.render(area, buf);
}

/// Renders a placeholder message when no step is selected.
///
/// # Arguments
///
/// * `area` - The area to render the placeholder within
/// * `buf` - The buffer to render to
/// * `colors` - The TUI color scheme to use
fn render_placeholder(area: Rect, buf: &mut Buffer, colors: &TuiColors) {
    let placeholder = Paragraph::new("Select a decoder step to view details")
        .style(colors.placeholder)
        .alignment(Alignment::Center);

    // Center vertically by calculating offset
    let vertical_center = area.height / 2;
    let centered_area = Rect {
        x: area.x,
        y: area.y + vertical_center,
        width: area.width,
        height: 1,
    };

    placeholder.render(centered_area, buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_text_short() {
        let text = "Hello World";
        let result = truncate_text(text, 200);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_truncate_text_long() {
        let text = "a".repeat(250);
        let result = truncate_text(&text, 200);
        assert_eq!(result.len(), 203); // 200 + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_text_exact_limit() {
        let text = "a".repeat(200);
        let result = truncate_text(&text, 200);
        assert_eq!(result.len(), 200);
        assert!(!result.ends_with("..."));
    }
}
