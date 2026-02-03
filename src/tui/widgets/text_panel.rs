//! Text panel widget for displaying scrollable, bordered text areas.
//!
//! This module provides a reusable widget for rendering titled, bordered
//! text panels with support for word wrapping, scrolling, and different
//! styling for success output.
//!
//! # Example
//!
//! ```ignore
//! use ratatui::prelude::*;
//! use ciphey::tui::widgets::text_panel::render_text_panel;
//! use ciphey::tui::colors::TuiColors;
//!
//! let colors = TuiColors::default();
//! let mut buf = Buffer::empty(Rect::new(0, 0, 30, 5));
//!
//! render_text_panel(
//!     Rect::new(0, 0, 30, 5),
//!     &mut buf,
//!     "Input",
//!     "SGVsbG8gV29ybGQ=",
//!     &colors,
//!     false,
//! );
//! ```

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::super::colors::TuiColors;

/// Placeholder text displayed when content is empty.
const EMPTY_PLACEHOLDER: &str = "(empty)";

/// Character used to represent non-printable characters.
const NON_PRINTABLE_REPLACEMENT: char = '\u{FFFD}'; // Unicode replacement character

/// A scrollable text panel widget with title and border.
///
/// `TextPanel` renders a bordered area with a configurable title,
/// supporting word wrapping and vertical scrolling for content that
/// exceeds the visible area.
///
/// # Example
///
/// ```ignore
/// use ratatui::prelude::*;
/// use ciphey::tui::widgets::text_panel::TextPanel;
/// use ciphey::tui::colors::TuiColors;
///
/// let colors = TuiColors::default();
/// let panel = TextPanel::new("Input", "SGVsbG8gV29ybGQ=");
/// panel.render(area, &mut buf, &colors);
/// ```
#[derive(Debug, Clone)]
pub struct TextPanel<'a> {
    /// The title displayed in the border.
    title: &'a str,
    /// The text content to display.
    content: &'a str,
    /// Vertical scroll offset (number of lines to skip from top).
    scroll_offset: u16,
    /// Whether to use success styling (green text).
    is_success: bool,
}

impl<'a> TextPanel<'a> {
    /// Creates a new `TextPanel` with the given title and content.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display in the panel border
    /// * `content` - The text content to display
    ///
    /// # Returns
    ///
    /// A new `TextPanel` instance with default settings (no scroll, not success).
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self {
            title,
            content,
            scroll_offset: 0,
            is_success: false,
        }
    }

    /// Sets the vertical scroll offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - Number of lines to scroll from the top
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn scroll(mut self, offset: u16) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Sets whether to use success styling (green text).
    ///
    /// # Arguments
    ///
    /// * `is_success` - If true, text will be rendered in success color
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn success(mut self, is_success: bool) -> Self {
        self.is_success = is_success;
        self
    }

    /// Renders the text panel to the given buffer area.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area to render into
    /// * `buf` - The buffer to render to
    /// * `colors` - The color scheme to use
    pub fn render(&self, area: Rect, buf: &mut Buffer, colors: &TuiColors) {
        render_text_panel_with_scroll(
            area,
            buf,
            self.title,
            self.content,
            colors,
            self.is_success,
            self.scroll_offset,
        );
    }
}

/// Sanitizes content by replacing non-printable characters with a placeholder.
///
/// This function preserves newlines, tabs, and standard printable characters
/// while replacing control characters and other non-printable characters
/// with the Unicode replacement character.
///
/// # Arguments
///
/// * `content` - The raw content string to sanitize
///
/// # Returns
///
/// A new string with non-printable characters replaced.
fn sanitize_content(content: &str) -> String {
    content
        .chars()
        .map(|c| {
            if c.is_control() && c != '\n' && c != '\t' && c != '\r' {
                NON_PRINTABLE_REPLACEMENT
            } else {
                c
            }
        })
        .collect()
}

/// Prepares content for display, handling empty content and sanitization.
///
/// # Arguments
///
/// * `content` - The raw content string
///
/// # Returns
///
/// A tuple of (display_string, is_placeholder) where is_placeholder indicates
/// if the returned string is the empty placeholder.
fn prepare_content(content: &str) -> (String, bool) {
    if content.is_empty() {
        (EMPTY_PLACEHOLDER.to_string(), true)
    } else {
        (sanitize_content(content), false)
    }
}

/// Renders a titled, bordered text panel.
///
/// This is the main rendering function for text panels. It handles:
/// - Bordered frame with title
/// - Word wrapping for long lines
/// - Vertical scrolling for content exceeding visible area
/// - Different styling for success output
/// - Empty content placeholder
/// - Non-printable character escaping
///
/// # Arguments
///
/// * `area` - The rectangular area to render into
/// * `buf` - The buffer to render to
/// * `title` - The title to display in the panel border
/// * `content` - The text content to display
/// * `colors` - The color scheme to use
/// * `is_success` - If true, use success color for text
///
/// # Example
///
/// ```ignore
/// use ratatui::prelude::*;
/// use ciphey::tui::widgets::text_panel::render_text_panel;
/// use ciphey::tui::colors::TuiColors;
///
/// let mut buf = Buffer::empty(Rect::new(0, 0, 30, 5));
/// let colors = TuiColors::default();
///
/// render_text_panel(
///     Rect::new(0, 0, 30, 5),
///     &mut buf,
///     "Input",
///     "SGVsbG8gV29ybGQ=",
///     &colors,
///     false,
/// );
/// ```
pub fn render_text_panel(
    area: Rect,
    buf: &mut Buffer,
    title: &str,
    content: &str,
    colors: &TuiColors,
    is_success: bool,
) {
    render_text_panel_with_scroll(area, buf, title, content, colors, is_success, 0);
}

/// Renders a titled, bordered text panel with scroll support.
///
/// Extended version of `render_text_panel` that supports vertical scrolling.
///
/// # Arguments
///
/// * `area` - The rectangular area to render into
/// * `buf` - The buffer to render to
/// * `title` - The title to display in the panel border
/// * `content` - The text content to display
/// * `colors` - The color scheme to use
/// * `is_success` - If true, use success color for text
/// * `scroll_offset` - Number of lines to scroll from the top
pub fn render_text_panel_with_scroll(
    area: Rect,
    buf: &mut Buffer,
    title: &str,
    content: &str,
    colors: &TuiColors,
    is_success: bool,
    scroll_offset: u16,
) {
    // Prepare and sanitize content
    let (display_content, is_placeholder) = prepare_content(content);

    // Determine text style based on success state and placeholder status
    let text_style = if is_placeholder {
        colors.muted
    } else if is_success {
        colors.success
    } else {
        colors.text
    };

    // Create the bordered block with title
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(colors.border)
        .title(format!(" {} ", title))
        .title_style(colors.highlight);

    // Create the paragraph with word wrapping and scroll
    let paragraph = Paragraph::new(display_content)
        .style(text_style)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));

    // Render to buffer
    paragraph.render(area, buf);
}

/// Calculates the total number of wrapped lines for given content and width.
///
/// This is useful for determining scroll limits.
///
/// # Arguments
///
/// * `content` - The text content
/// * `width` - The available width for text (excluding borders)
///
/// # Returns
///
/// The total number of lines after word wrapping.
pub fn calculate_line_count(content: &str, width: u16) -> usize {
    if content.is_empty() {
        return 1; // Placeholder takes one line
    }

    // Account for border padding (2 chars on each side)
    let effective_width = width.saturating_sub(2) as usize;
    if effective_width == 0 {
        return content.lines().count().max(1);
    }

    content
        .lines()
        .map(|line| {
            if line.is_empty() {
                1
            } else {
                // Calculate how many visual lines this line will take
                (line.chars().count() + effective_width - 1) / effective_width
            }
        })
        .sum()
}

/// Calculates the maximum scroll offset for given content and visible area.
///
/// # Arguments
///
/// * `content` - The text content
/// * `area_width` - The total width of the panel area
/// * `area_height` - The total height of the panel area
///
/// # Returns
///
/// The maximum valid scroll offset (0 if content fits).
pub fn max_scroll_offset(content: &str, area_width: u16, area_height: u16) -> u16 {
    // Account for borders (1 line top, 1 line bottom)
    let visible_lines = area_height.saturating_sub(2) as usize;
    let total_lines = calculate_line_count(content, area_width);

    if total_lines <= visible_lines {
        0
    } else {
        (total_lines - visible_lines) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_content_preserves_printable() {
        let input = "Hello, World!";
        let result = sanitize_content(input);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_sanitize_content_preserves_newlines() {
        let input = "Line 1\nLine 2\nLine 3";
        let result = sanitize_content(input);
        assert_eq!(result, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_sanitize_content_preserves_tabs() {
        let input = "Column1\tColumn2";
        let result = sanitize_content(input);
        assert_eq!(result, "Column1\tColumn2");
    }

    #[test]
    fn test_sanitize_content_replaces_control_chars() {
        let input = "Hello\x00World\x07!";
        let result = sanitize_content(input);
        assert!(result.contains(NON_PRINTABLE_REPLACEMENT));
        assert!(!result.contains('\x00'));
        assert!(!result.contains('\x07'));
    }

    #[test]
    fn test_prepare_content_empty() {
        let (content, is_placeholder) = prepare_content("");
        assert_eq!(content, EMPTY_PLACEHOLDER);
        assert!(is_placeholder);
    }

    #[test]
    fn test_prepare_content_non_empty() {
        let (content, is_placeholder) = prepare_content("Test content");
        assert_eq!(content, "Test content");
        assert!(!is_placeholder);
    }

    #[test]
    fn test_calculate_line_count_empty() {
        assert_eq!(calculate_line_count("", 80), 1);
    }

    #[test]
    fn test_calculate_line_count_single_line() {
        assert_eq!(calculate_line_count("Hello", 80), 1);
    }

    #[test]
    fn test_calculate_line_count_multiple_lines() {
        let content = "Line 1\nLine 2\nLine 3";
        assert_eq!(calculate_line_count(content, 80), 3);
    }

    #[test]
    fn test_calculate_line_count_wrapping() {
        // 10 chars per line (width 12 - 2 for borders)
        let content = "12345678901234567890"; // 20 chars should wrap to 2 lines
        assert_eq!(calculate_line_count(content, 12), 2);
    }

    #[test]
    fn test_max_scroll_offset_content_fits() {
        let content = "Short";
        // Area height 10, minus 2 for borders = 8 visible lines
        assert_eq!(max_scroll_offset(content, 80, 10), 0);
    }

    #[test]
    fn test_max_scroll_offset_content_exceeds() {
        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        // Area height 5, minus 2 for borders = 3 visible lines
        // 5 total lines - 3 visible = 2 max scroll
        assert_eq!(max_scroll_offset(content, 80, 5), 2);
    }

    #[test]
    fn test_text_panel_builder() {
        let panel = TextPanel::new("Test", "Content").scroll(5).success(true);

        assert_eq!(panel.title, "Test");
        assert_eq!(panel.content, "Content");
        assert_eq!(panel.scroll_offset, 5);
        assert!(panel.is_success);
    }

    #[test]
    fn test_text_panel_default_values() {
        let panel = TextPanel::new("Title", "Body");

        assert_eq!(panel.scroll_offset, 0);
        assert!(!panel.is_success);
    }

    #[test]
    fn test_render_text_panel_creates_output() {
        let colors = TuiColors::default();
        let area = Rect::new(0, 0, 30, 5);
        let mut buf = Buffer::empty(area);

        render_text_panel(area, &mut buf, "Input", "Hello World", &colors, false);

        // Verify that the buffer is not empty (has been written to)
        // Check that the title appears in the output
        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("Input"));
    }

    #[test]
    fn test_render_text_panel_success_styling() {
        let colors = TuiColors::default();
        let area = Rect::new(0, 0, 30, 5);
        let mut buf = Buffer::empty(area);

        render_text_panel(area, &mut buf, "Output", "Success!", &colors, true);

        // Verify that the buffer contains the content
        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("Output"));
    }

    #[test]
    fn test_render_text_panel_empty_content_placeholder() {
        let colors = TuiColors::default();
        let area = Rect::new(0, 0, 30, 5);
        let mut buf = Buffer::empty(area);

        render_text_panel(area, &mut buf, "Empty", "", &colors, false);

        // Verify that the placeholder text appears
        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("empty"));
    }
}
