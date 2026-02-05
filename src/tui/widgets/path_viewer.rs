//! Path viewer widget for rendering the decoder chain.
//!
//! This module provides a widget for visualizing the sequence of decoders
//! applied during the cracking process as a horizontal chain of linked boxes.

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::decoders::crack_results::CrackResult;

use super::super::colors::TuiColors;

/// Width of each decoder box (including borders).
const BOX_WIDTH: u16 = 12;

/// Height of each decoder box (including borders).
const BOX_HEIGHT: u16 = 3;

/// Height of the key line below the box (if present).
const KEY_LINE_HEIGHT: u16 = 1;

/// Width of the arrow connector between boxes.
const ARROW_WIDTH: u16 = 5;

/// The arrow string used between decoder boxes.
const ARROW_STR: &str = " --> ";

/// Widget for rendering a decoder path as a horizontal chain of linked boxes.
///
/// Displays each decoder in the path as a bordered box with arrows between them.
/// The currently selected decoder is highlighted with a different border style.
///
/// # Example Output
///
/// ```text
/// +----------+     +----------+     +----------+
/// |  Base64  | --> |  Base64  | --> |  Caesar  |
/// +----------+     +----------+     +----------+
///                                      key: 13
///                                    (selected)
/// ```
pub struct PathViewer;

impl PathViewer {
    /// Creates a new `PathViewer` instance.
    pub fn new() -> Self {
        Self
    }

    /// Renders the decoder path chain into the given buffer area.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area to render into
    /// * `buf` - The buffer to render into
    /// * `path` - A slice of `CrackResult` representing the decoder path
    /// * `selected` - The index of the currently selected step
    /// * `colors` - The TUI color scheme to use
    ///
    /// # Rendering Details
    ///
    /// - Empty paths display "No decoders used"
    /// - Paths that are too long for the screen are truncated with ellipsis
    ///   and centered around the selected item
    /// - Each decoder is shown in a bordered box with arrows between them
    /// - The selected decoder has a highlighted border
    /// - If a decoder has a key, it's displayed below the box
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        path: &[CrackResult],
        selected: usize,
        colors: &TuiColors,
    ) {
        // Handle empty path case
        if path.is_empty() {
            let msg = Paragraph::new("No decoders used")
                .style(colors.text_dimmed)
                .alignment(Alignment::Center);
            let centered_area = centered_rect(area, 20, 1);
            msg.render(centered_area, buf);
            return;
        }

        // Calculate how many boxes we can fit
        let total_width_per_box = BOX_WIDTH + ARROW_WIDTH;
        let available_width = area.width.saturating_sub(BOX_WIDTH); // Last box doesn't need arrow
        let max_visible = if available_width == 0 {
            1
        } else {
            (available_width / total_width_per_box) as usize + 1
        };

        // Determine which range of path items to display
        let (start_idx, end_idx, show_left_ellipsis, show_right_ellipsis) =
            calculate_visible_range(path.len(), selected, max_visible);

        // Calculate starting x position to center the chain
        let visible_count = end_idx - start_idx;
        let chain_width =
            calculate_chain_width(visible_count, show_left_ellipsis, show_right_ellipsis);
        let start_x = area.x + area.width.saturating_sub(chain_width) / 2;

        let mut current_x = start_x;
        let box_y = area.y + (area.height.saturating_sub(BOX_HEIGHT + KEY_LINE_HEIGHT + 1)) / 2;

        // Draw left ellipsis if needed
        if show_left_ellipsis {
            self.render_ellipsis(current_x, box_y, buf, colors);
            current_x += 5; // "... " width
        }

        // Draw each visible decoder box
        for (display_idx, path_idx) in (start_idx..end_idx).enumerate() {
            let crack_result = &path[path_idx];
            let is_selected = path_idx == selected;

            // Draw the decoder box
            self.render_decoder_box(current_x, box_y, buf, crack_result, is_selected, colors);

            // Draw key below box if present
            if let Some(ref key) = crack_result.key {
                self.render_key_label(current_x, box_y + BOX_HEIGHT, buf, key, colors);
            }

            current_x += BOX_WIDTH;

            // Draw arrow if not the last visible box
            if display_idx < visible_count - 1 || show_right_ellipsis {
                self.render_arrow(current_x, box_y + 1, buf, colors);
                current_x += ARROW_WIDTH;
            }
        }

        // Draw right ellipsis if needed
        if show_right_ellipsis {
            self.render_ellipsis(current_x, box_y, buf, colors);
        }
    }

    /// Renders a single decoder box.
    fn render_decoder_box(
        &self,
        x: u16,
        y: u16,
        buf: &mut Buffer,
        crack_result: &CrackResult,
        is_selected: bool,
        colors: &TuiColors,
    ) {
        let box_area = Rect::new(x, y, BOX_WIDTH, BOX_HEIGHT);

        // Choose border style based on selection
        let border_style = if is_selected {
            colors.accent
        } else {
            colors.border
        };

        let border_type = if is_selected {
            symbols::border::DOUBLE
        } else {
            symbols::border::PLAIN
        };

        // Create the box with border
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_set(border_type);

        // Truncate decoder name if too long
        let max_name_len = (BOX_WIDTH - 2) as usize;
        let decoder_name = truncate_string(crack_result.decoder, max_name_len);

        // Create paragraph with decoder name
        let text_style = if is_selected {
            colors
                .accent
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
        } else {
            colors.text
        };

        let paragraph = Paragraph::new(decoder_name)
            .style(text_style)
            .alignment(Alignment::Center)
            .block(block);

        paragraph.render(box_area, buf);
    }

    /// Renders the arrow connector between boxes.
    fn render_arrow(&self, x: u16, y: u16, buf: &mut Buffer, colors: &TuiColors) {
        buf.set_string(x, y, ARROW_STR, colors.text_dimmed);
    }

    /// Renders the key label below a decoder box.
    fn render_key_label(&self, x: u16, y: u16, buf: &mut Buffer, key: &str, colors: &TuiColors) {
        let key_str = format!("key: {}", key);
        let max_len = BOX_WIDTH as usize;
        let display_str = truncate_string(&key_str, max_len);

        // Center the key string under the box
        let padding = (BOX_WIDTH as usize).saturating_sub(display_str.len()) / 2;

        buf.set_string(x + padding as u16, y, &display_str, colors.info);
    }

    /// Renders an ellipsis indicator for truncated paths.
    fn render_ellipsis(&self, x: u16, y: u16, buf: &mut Buffer, colors: &TuiColors) {
        buf.set_string(x, y + 1, "...", colors.text_dimmed);
    }
}

impl Default for PathViewer {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculates the visible range of path items to display.
///
/// Returns (start_index, end_index, show_left_ellipsis, show_right_ellipsis).
fn calculate_visible_range(
    total: usize,
    selected: usize,
    max_visible: usize,
) -> (usize, usize, bool, bool) {
    if total <= max_visible {
        // All items fit
        return (0, total, false, false);
    }

    // Center around the selected item
    let half = max_visible / 2;

    // Calculate ideal start position
    let ideal_start = selected.saturating_sub(half);
    let ideal_end = ideal_start + max_visible;

    // Adjust if we're past the end
    let (start, end) = if ideal_end > total {
        let start = total.saturating_sub(max_visible);
        (start, total)
    } else {
        (ideal_start, ideal_end)
    };

    let show_left_ellipsis = start > 0;
    let show_right_ellipsis = end < total;

    (start, end, show_left_ellipsis, show_right_ellipsis)
}

/// Calculates the total width of the visible chain.
fn calculate_chain_width(visible_count: usize, left_ellipsis: bool, right_ellipsis: bool) -> u16 {
    if visible_count == 0 {
        return 0;
    }

    let mut width = BOX_WIDTH * visible_count as u16;
    // Add arrows between boxes
    if visible_count > 1 {
        width += ARROW_WIDTH * (visible_count as u16 - 1);
    }

    // Add ellipsis widths
    if left_ellipsis {
        width += 5; // "... " + spacing
    }
    if right_ellipsis {
        width += ARROW_WIDTH + 3; // arrow + "..."
    }

    width
}

/// Truncates a string to fit within the given length.
///
/// If the string is longer than `max_len`, it's truncated and "..." is appended.
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Creates a centered rectangle within the given area.
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_string_short() {
        assert_eq!(truncate_string("Base64", 10), "Base64");
    }

    #[test]
    fn test_truncate_string_exact() {
        assert_eq!(truncate_string("Base64", 6), "Base64");
    }

    #[test]
    fn test_truncate_string_long() {
        assert_eq!(truncate_string("VeryLongDecoderName", 10), "VeryLon...");
    }

    #[test]
    fn test_truncate_string_tiny() {
        assert_eq!(truncate_string("Test", 2), "..");
    }

    #[test]
    fn test_calculate_visible_range_all_fit() {
        let (start, end, left, right) = calculate_visible_range(3, 1, 5);
        assert_eq!((start, end), (0, 3));
        assert!(!left);
        assert!(!right);
    }

    #[test]
    fn test_calculate_visible_range_centered() {
        let (start, end, left, right) = calculate_visible_range(10, 5, 3);
        assert_eq!((start, end), (4, 7));
        assert!(left);
        assert!(right);
    }

    #[test]
    fn test_calculate_visible_range_at_start() {
        let (start, end, left, right) = calculate_visible_range(10, 0, 3);
        assert_eq!((start, end), (0, 3));
        assert!(!left);
        assert!(right);
    }

    #[test]
    fn test_calculate_visible_range_at_end() {
        let (start, end, left, right) = calculate_visible_range(10, 9, 3);
        assert_eq!((start, end), (7, 10));
        assert!(left);
        assert!(!right);
    }

    #[test]
    fn test_path_viewer_new() {
        let viewer = PathViewer::new();
        // Just verify it compiles and creates
        let _ = viewer;
    }

    #[test]
    fn test_path_viewer_default() {
        let viewer = PathViewer::default();
        let _ = viewer;
    }

    #[test]
    fn test_calculate_chain_width() {
        // Single box
        assert_eq!(calculate_chain_width(1, false, false), BOX_WIDTH);

        // Two boxes with arrow
        assert_eq!(
            calculate_chain_width(2, false, false),
            BOX_WIDTH * 2 + ARROW_WIDTH
        );

        // Three boxes with arrows
        assert_eq!(
            calculate_chain_width(3, false, false),
            BOX_WIDTH * 3 + ARROW_WIDTH * 2
        );
    }
}
