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
/// Uses char count for UTF-8 safe truncation.
///
/// # Arguments
///
/// * `text` - The text to potentially truncate
/// * `max_len` - Maximum allowed character count before truncation
///
/// # Returns
///
/// The original text if within limit, or truncated text with "..." appended.
fn truncate_text(text: &str, max_len: usize) -> String {
    let char_count = text.chars().count();
    if char_count <= max_len {
        text.to_string()
    } else {
        format!("{}...", text.chars().take(max_len).collect::<String>())
    }
}

/// Calculates metadata for a text string including char count, byte size, line count, and printable percentage.
///
/// This provides useful information about the text being processed through each decoder step,
/// helping users understand the transformation at each stage.
///
/// # Arguments
///
/// * `text` - The text to analyze
///
/// # Returns
///
/// A formatted string like "42 chars, 42 bytes, 1 line, 100% printable"
///
/// # Example
///
/// ```ignore
/// let metadata = calculate_text_metadata("hello world");
/// // Returns: "11 chars, 11 bytes, 1 line, 100% printable"
/// ```
fn calculate_text_metadata(text: &str) -> String {
    let char_count = text.chars().count();
    let byte_size = text.len();
    let line_count = text.lines().count().max(1); // At least 1 line even if empty

    // Count printable characters (excluding control chars except newline/tab/carriage return)
    let printable_count = text
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t' || *c == '\r')
        .count();

    let printable_pct = if char_count > 0 {
        (printable_count * 100) / char_count
    } else {
        100
    };

    format!(
        "{} chars, {} bytes, {} line{}, {}% printable",
        char_count,
        byte_size,
        line_count,
        if line_count == 1 { "" } else { "s" },
        printable_pct
    )
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

    // Calculate metadata for input
    let input_metadata = calculate_text_metadata(&result.encrypted_text);
    let input_label = format!("Input to this step ({})", input_metadata);

    // Calculate metadata for output (only if not "N/A")
    let output_label = result
        .unencrypted_text
        .as_ref()
        .filter(|texts| !texts.is_empty())
        .map(|texts| {
            let full_output = texts.join(", ");
            format!(
                "Output from this step ({})",
                calculate_text_metadata(&full_output)
            )
        })
        .unwrap_or_else(|| "Output from this step".to_string());

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
        Line::from(vec![Span::styled(
            input_label,
            colors.label.add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ", colors.text), // Indentation
            Span::styled(before_text, colors.text_before),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            output_label,
            colors.label.add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ", colors.text), // Indentation
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
        // 200 chars + "..." = 203 chars
        assert_eq!(result.chars().count(), 203);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_text_exact_limit() {
        let text = "a".repeat(200);
        let result = truncate_text(&text, 200);
        assert_eq!(result.chars().count(), 200);
        assert!(!result.ends_with("..."));
    }

    #[test]
    fn test_truncate_text_unicode_safe() {
        // Test that multi-byte characters don't cause panic
        let text = "世界".repeat(100); // 200 chars, but many more bytes
        let result = truncate_text(&text, 50);
        assert_eq!(result.chars().count(), 53); // 50 chars + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_calculate_text_metadata_simple() {
        let result = calculate_text_metadata("hello");
        assert!(result.contains("5 chars"));
        assert!(result.contains("5 bytes"));
        assert!(result.contains("1 line"));
        assert!(result.contains("100% printable"));
    }

    #[test]
    fn test_calculate_text_metadata_multiline() {
        let result = calculate_text_metadata("line1\nline2\nline3");
        assert!(result.contains("17 chars"));
        assert!(result.contains("17 bytes"));
        assert!(result.contains("3 lines"));
    }

    #[test]
    fn test_calculate_text_metadata_with_control_chars() {
        let text = "hello\x00world";
        let result = calculate_text_metadata(text);
        // Should show less than 100% printable due to null byte
        assert!(result.contains("11 chars"));
        assert!(result.contains("11 bytes"));
        // 10 printable chars out of 11 = 90%
        assert!(result.contains("90% printable"));
    }

    #[test]
    fn test_calculate_text_metadata_empty() {
        let result = calculate_text_metadata("");
        assert!(result.contains("0 chars"));
        assert!(result.contains("0 bytes"));
        assert!(result.contains("1 line")); // Empty text is still 1 line
        assert!(result.contains("100% printable"));
    }

    #[test]
    fn test_calculate_text_metadata_unicode() {
        let result = calculate_text_metadata("hello 世界");
        assert!(result.contains("8 chars")); // 5 + space + 2 CJK characters
        assert!(result.contains("12 bytes")); // ASCII + UTF-8 encoded CJK
        assert!(result.contains("100% printable"));
    }

    #[test]
    fn test_calculate_text_metadata_tabs_and_newlines() {
        let result = calculate_text_metadata("hello\tworld\n");
        // Tab and newline should count as printable
        assert!(result.contains("12 chars"));
        assert!(result.contains("100% printable"));
        assert!(result.contains("1 line")); // Trailing newline doesn't create a new line in lines()
    }
}
