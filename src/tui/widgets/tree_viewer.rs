//! Tree viewer widget for rendering a birds-eye view of the decoder tree.
//!
//! This module provides a widget for visualizing the entire decoder tree,
//! including the main decoding path and all branches forking off from it.
//! The main path is rendered as a horizontal chain of boxes connected by
//! arrows, while branches
//! are shown as compact nodes beneath their parent step.

use std::collections::HashMap;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::decoders::crack_results::CrackResult;

use super::super::colors::TuiColors;

/// Width of each decoder box in the main path (including borders).
const BOX_WIDTH: u16 = 12;

/// Height of each decoder box in the main path (including borders).
const BOX_HEIGHT: u16 = 3;

/// Width of the arrow connector between main path boxes.
const ARROW_WIDTH: u16 = 5;

/// The arrow string used between decoder boxes in the main path.
const ARROW_STR: &str = " --> ";

/// Prefix string for branch nodes (tree connector).
const BRANCH_PREFIX: &str = "+-- ";

/// A node in the tree for rendering purposes.
///
/// Represents a single branch off the main decoder path, containing
/// enough information to render a compact summary in the tree view.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Decoder name for this node.
    pub decoder_name: String,
    /// Whether this node has sub-branches (shows "+" indicator).
    pub has_children: bool,
    /// Whether this branch was successful.
    pub successful: bool,
    /// Cache ID for this branch (`None` for main path nodes).
    pub cache_id: Option<i64>,
    /// Preview of the final decoded text.
    pub text_preview: String,
}

/// Widget for rendering a birds-eye tree view of the decoder path and all branches.
///
/// Displays the main decoding path as a horizontal chain of bordered boxes
/// with arrows, plus all branches forking off from each step. Branch nodes
/// are rendered as compact text lines beneath their parent in the main path.
///
/// # Example Output
///
/// ```text
/// [Input] --> [Base64] --> [Caesar] --> [Plaintext]
///                |
///                +-- [Hex]+
///                +-- [ROT13]
///                +-- [Reverse]+
/// ```
pub struct TreeViewer;

impl TreeViewer {
    /// Creates a new `TreeViewer` instance.
    pub fn new() -> Self {
        Self
    }

    /// Renders the decoder tree into the given buffer area.
    ///
    /// This draws the main decoder path as a horizontal chain of boxes,
    /// then draws vertical connectors and branch nodes beneath each step
    /// that has branches.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area to render into
    /// * `buf` - The buffer to render into
    /// * `path` - A slice of `CrackResult` representing the main decoder path
    /// * `selected_step` - The index of the currently selected step in the main path
    /// * `branches_by_step` - Branches at each step index, keyed by step index
    /// * `colors` - The TUI color scheme to use
    ///
    /// # Rendering Details
    ///
    /// - Empty paths display "No decoders used"
    /// - Paths that are too wide are truncated with `...` ellipsis, centered
    ///   around the selected step
    /// - Selected step uses accent color with bold, reversed, and double border
    /// - Branch nodes use compact `+-- [Name]+` format beneath their parent
    /// - If branches exceed available vertical space, a scroll indicator is shown
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        path: &[CrackResult],
        selected_step: usize,
        branches_by_step: &HashMap<usize, Vec<TreeNode>>,
        colors: &TuiColors,
    ) {
        // Handle empty path case
        if path.is_empty() {
            let msg = Paragraph::new("No decoders used")
                .style(colors.muted)
                .alignment(Alignment::Center);
            let centered_area = centered_rect(area, 20, 1);
            msg.render(centered_area, buf);
            return;
        }

        // Calculate how many boxes we can fit horizontally
        let total_width_per_box = BOX_WIDTH + ARROW_WIDTH;
        let available_width = area.width.saturating_sub(BOX_WIDTH);
        let max_visible = if available_width == 0 {
            1
        } else {
            (available_width / total_width_per_box) as usize + 1
        };

        // Determine visible range centered around the selected step
        let (start_idx, end_idx, show_left_ellipsis, show_right_ellipsis) =
            calculate_visible_range(path.len(), selected_step, max_visible);

        // Calculate starting x position to center the chain
        let visible_count = end_idx - start_idx;
        let chain_width =
            calculate_chain_width(visible_count, show_left_ellipsis, show_right_ellipsis);
        let start_x = area.x + area.width.saturating_sub(chain_width) / 2;

        let mut current_x = start_x;
        let box_y = area.y;

        // Track x positions for each visible path index (for drawing branch connectors)
        let mut box_positions: HashMap<usize, u16> = HashMap::new();

        // Draw left ellipsis if needed
        if show_left_ellipsis {
            self.render_ellipsis(current_x, box_y, buf, colors);
            current_x += 5; // "... " width
        }

        // Draw each visible decoder box in the main path
        for (display_idx, path_idx) in (start_idx..end_idx).enumerate() {
            let crack_result = &path[path_idx];
            let is_selected = path_idx == selected_step;

            box_positions.insert(path_idx, current_x);

            self.render_decoder_box(current_x, box_y, buf, crack_result, is_selected, colors);

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

        // Draw branches beneath their parent steps
        let branch_start_y = box_y + BOX_HEIGHT;
        let available_branch_rows =
            area.height.saturating_sub(BOX_HEIGHT).saturating_sub(1) as usize; // -1 for the connector line

        for (&step_idx, branches) in branches_by_step {
            if let Some(&parent_x) = box_positions.get(&step_idx) {
                self.render_branches(
                    parent_x,
                    branch_start_y,
                    buf,
                    branches,
                    available_branch_rows,
                    step_idx == selected_step,
                    colors,
                );
            }
        }
    }

    /// Renders a single decoder box in the main path.
    ///
    /// Selected boxes use accent color with bold/reversed text and a double border.
    /// Unselected boxes use normal text with a plain border.
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

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_set(border_type);

        let max_name_len = (BOX_WIDTH - 2) as usize;
        let decoder_name = truncate_string(crack_result.decoder, max_name_len);

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

    /// Renders the arrow connector between main path boxes.
    fn render_arrow(&self, x: u16, y: u16, buf: &mut Buffer, colors: &TuiColors) {
        buf.set_string(x, y, ARROW_STR, colors.muted);
    }

    /// Renders an ellipsis indicator for truncated paths.
    fn render_ellipsis(&self, x: u16, y: u16, buf: &mut Buffer, colors: &TuiColors) {
        buf.set_string(x, y + 1, "...", colors.muted);
    }

    /// Renders branches beneath a parent step in the main path.
    ///
    /// Draws a vertical connector line from the parent box down to the
    /// branch list, then renders each branch as a compact `+-- [Name]+` line.
    /// If there are more branches than available rows, a scroll indicator
    /// (`... N more`) is shown at the bottom.
    ///
    /// # Arguments
    ///
    /// * `parent_x` - X position of the parent decoder box
    /// * `start_y` - Y position to start rendering branches
    /// * `buf` - The buffer to render into
    /// * `branches` - The branch nodes to render
    /// * `max_rows` - Maximum number of rows available for branches
    /// * `is_selected_parent` - Whether the parent step is currently selected
    /// * `colors` - The TUI color scheme to use
    fn render_branches(
        &self,
        parent_x: u16,
        start_y: u16,
        buf: &mut Buffer,
        branches: &[TreeNode],
        max_rows: usize,
        is_selected_parent: bool,
        colors: &TuiColors,
    ) {
        if branches.is_empty() || max_rows == 0 {
            return;
        }

        // Draw vertical connector from the parent box center
        let connector_x = parent_x + BOX_WIDTH / 2;

        // Reserve 1 row for the connector pipe, and potentially 1 for scroll indicator
        let connector_row = start_y;
        buf.set_string(
            connector_x,
            connector_row,
            "|",
            if is_selected_parent {
                colors.accent
            } else {
                colors.muted
            },
        );

        let branch_start_y = connector_row + 1;
        let rows_for_branches = max_rows.saturating_sub(1); // subtract the connector row

        if rows_for_branches == 0 {
            return;
        }

        // Determine how many branches to show
        let show_scroll_indicator = branches.len() > rows_for_branches;
        let visible_count = if show_scroll_indicator {
            rows_for_branches.saturating_sub(1) // leave room for "... N more"
        } else {
            branches.len()
        };

        // Render visible branch nodes
        for (i, branch) in branches.iter().take(visible_count).enumerate() {
            let row_y = branch_start_y + i as u16;
            self.render_branch_node(parent_x, row_y, buf, branch, colors);
        }

        // Render scroll indicator if needed
        if show_scroll_indicator {
            let remaining = branches.len() - visible_count;
            let indicator = format!("... {} more", remaining);
            let indicator_y = branch_start_y + visible_count as u16;
            buf.set_string(parent_x, indicator_y, &indicator, colors.muted);
        }
    }

    /// Renders a single branch node as a compact text line.
    ///
    /// Format: `+-- [Name]+` where the trailing `+` indicates `has_children`.
    /// Successful branches use the success color style.
    fn render_branch_node(
        &self,
        x: u16,
        y: u16,
        buf: &mut Buffer,
        node: &TreeNode,
        colors: &TuiColors,
    ) {
        let children_indicator = if node.has_children { "+" } else { "" };
        let branch_text = format!(
            "{}[{}]{}",
            BRANCH_PREFIX, node.decoder_name, children_indicator
        );

        let style = if node.successful {
            colors.success
        } else {
            colors.text
        };

        buf.set_string(x, y, &branch_text, style);
    }
}

impl Default for TreeViewer {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculates the visible range of path items to display.
///
/// Centers the visible window around the selected item. Returns a tuple of
/// `(start_index, end_index, show_left_ellipsis, show_right_ellipsis)`.
fn calculate_visible_range(
    total: usize,
    selected: usize,
    max_visible: usize,
) -> (usize, usize, bool, bool) {
    if total <= max_visible {
        return (0, total, false, false);
    }

    let half = max_visible / 2;
    let ideal_start = selected.saturating_sub(half);
    let ideal_end = ideal_start + max_visible;

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

/// Calculates the total width of the visible main path chain.
///
/// Accounts for box widths, arrow widths, and optional ellipsis indicators
/// on either side.
fn calculate_chain_width(visible_count: usize, left_ellipsis: bool, right_ellipsis: bool) -> u16 {
    if visible_count == 0 {
        return 0;
    }

    let mut width = BOX_WIDTH * visible_count as u16;
    if visible_count > 1 {
        width += ARROW_WIDTH * (visible_count as u16 - 1);
    }

    if left_ellipsis {
        width += 5; // "... " + spacing
    }
    if right_ellipsis {
        width += ARROW_WIDTH + 3; // arrow + "..."
    }

    width
}

/// Truncates a string to fit within the given maximum character length.
///
/// Uses `chars().count()` for UTF-8 safe truncation. If the string exceeds
/// `max_len`, it is truncated and `"..."` is appended.
fn truncate_string(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        format!("{}...", s.chars().take(max_len - 3).collect::<String>())
    }
}

/// Creates a centered rectangle within the given area.
///
/// If the requested width or height exceeds the area, they are clamped.
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_viewer_new() {
        let viewer = TreeViewer::new();
        let _ = viewer;
    }

    #[test]
    fn test_tree_viewer_default() {
        let viewer = TreeViewer::default();
        let _ = viewer;
    }

    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode {
            decoder_name: "Base64".to_string(),
            has_children: true,
            successful: false,
            cache_id: Some(42),
            text_preview: "aGVsbG8=".to_string(),
        };
        assert_eq!(node.decoder_name, "Base64");
        assert!(node.has_children);
        assert!(!node.successful);
        assert_eq!(node.cache_id, Some(42));
        assert_eq!(node.text_preview, "aGVsbG8=");
    }

    #[test]
    fn test_tree_node_without_cache_id() {
        let node = TreeNode {
            decoder_name: "Caesar".to_string(),
            has_children: false,
            successful: true,
            cache_id: None,
            text_preview: "hello world".to_string(),
        };
        assert_eq!(node.decoder_name, "Caesar");
        assert!(!node.has_children);
        assert!(node.successful);
        assert!(node.cache_id.is_none());
    }

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
    fn test_truncate_string_unicode() {
        let text = "世界Hello"; // 7 chars: 2 CJK + 5 ASCII
        assert_eq!(truncate_string(text, 5), "世界...");
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
    fn test_calculate_chain_width_single_box() {
        assert_eq!(calculate_chain_width(1, false, false), BOX_WIDTH);
    }

    #[test]
    fn test_calculate_chain_width_two_boxes() {
        assert_eq!(
            calculate_chain_width(2, false, false),
            BOX_WIDTH * 2 + ARROW_WIDTH
        );
    }

    #[test]
    fn test_calculate_chain_width_with_ellipsis() {
        let width = calculate_chain_width(2, true, true);
        let expected = BOX_WIDTH * 2 + ARROW_WIDTH + 5 + ARROW_WIDTH + 3;
        assert_eq!(width, expected);
    }

    #[test]
    fn test_calculate_chain_width_zero() {
        assert_eq!(calculate_chain_width(0, false, false), 0);
    }
}
