//! Multi-line text input component for TUI.
//!
//! This module provides a `MultilineTextInput` struct that supports multi-line
//! text editing, used for the home screen ciphertext input.
//!
//! Single-line byte-position calculations are delegated to
//! [`super::text_input::char_to_byte_pos`] to avoid code duplication.

use super::text_input::char_to_byte_pos;

/// A multi-line text input field with cursor management.
///
/// This struct handles:
/// - Character insertion at cursor position
/// - Newline insertion (via Ctrl+Enter)
/// - Backspace/delete operations across lines
/// - Cursor movement (including up/down between lines)
/// - Text retrieval and modification
#[derive(Debug, Clone, PartialEq)]
pub struct MultilineTextInput {
    /// The lines of text. Always has at least one line.
    lines: Vec<String>,
    /// Current cursor line (0-indexed).
    cursor_line: usize,
    /// Current cursor column within the line (in characters, not bytes).
    cursor_col: usize,
    /// Vertical scroll offset for display.
    scroll_offset: usize,
}

impl MultilineTextInput {
    /// Creates a new empty `MultilineTextInput`.
    ///
    /// # Returns
    ///
    /// A new `MultilineTextInput` with one empty line and cursor at (0, 0).
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }

    /// Inserts a character at the current cursor position.
    ///
    /// The cursor advances by one column after insertion.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to insert
    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cursor_line];
        let byte_pos = char_to_byte_pos(line, self.cursor_col);
        line.insert(byte_pos, c);
        self.cursor_col += 1;
    }

    /// Inserts a newline at the current cursor position.
    ///
    /// The text after the cursor is moved to a new line, and the cursor
    /// moves to the beginning of that new line.
    pub fn insert_newline(&mut self) {
        let byte_pos = char_to_byte_pos(&self.lines[self.cursor_line], self.cursor_col);

        // Split the current line
        let remainder = self.lines[self.cursor_line][byte_pos..].to_string();
        self.lines[self.cursor_line].truncate(byte_pos);

        // Insert the new line
        self.cursor_line += 1;
        self.lines.insert(self.cursor_line, remainder);
        self.cursor_col = 0;
    }

    /// Deletes the character before the cursor (backspace).
    ///
    /// If at the beginning of a line (not the first), joins with the previous line.
    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            // Delete character in current line
            self.cursor_col -= 1;
            let byte_pos = char_to_byte_pos(&self.lines[self.cursor_line], self.cursor_col);
            self.lines[self.cursor_line].remove(byte_pos);
        } else if self.cursor_line > 0 {
            // Join with previous line
            let current_line = self.lines.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].chars().count();
            self.lines[self.cursor_line].push_str(&current_line);
        }
    }

    /// Deletes the character at the cursor (delete key).
    ///
    /// If at the end of a line (not the last), joins with the next line.
    pub fn delete(&mut self) {
        let line_len = self.lines[self.cursor_line].chars().count();

        if self.cursor_col < line_len {
            // Delete character at cursor
            let byte_pos = char_to_byte_pos(&self.lines[self.cursor_line], self.cursor_col);
            self.lines[self.cursor_line].remove(byte_pos);
        } else if self.cursor_line < self.lines.len() - 1 {
            // Join with next line
            let next_line = self.lines.remove(self.cursor_line + 1);
            self.lines[self.cursor_line].push_str(&next_line);
        }
    }

    /// Clears all text and resets cursor to (0, 0).
    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
        self.cursor_line = 0;
        self.cursor_col = 0;
        self.scroll_offset = 0;
    }

    /// Sets the text content, parsing newlines into separate lines.
    ///
    /// Moves cursor to the end of the text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to set (may contain newlines)
    pub fn set_text(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.lines = if text.is_empty() {
            vec![String::new()]
        } else {
            text.lines().map(|s| s.to_string()).collect()
        };

        // Ensure at least one line
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        // Move cursor to end
        self.cursor_line = self.lines.len() - 1;
        self.cursor_col = self.lines[self.cursor_line].chars().count();
        self.scroll_offset = 0;
    }

    /// Gets the full text content as a single string with newlines.
    ///
    /// # Returns
    ///
    /// The text with lines joined by `\n`.
    pub fn get_text(&self) -> String {
        self.lines.join("\n")
    }

    /// Gets the lines as a slice.
    ///
    /// # Returns
    ///
    /// A reference to the lines vector.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Gets the current cursor position as (line, column).
    ///
    /// # Returns
    ///
    /// A tuple of (line_index, column_index).
    pub fn cursor_pos(&self) -> (usize, usize) {
        (self.cursor_line, self.cursor_col)
    }

    /// Gets the current scroll offset.
    ///
    /// # Returns
    ///
    /// The vertical scroll offset.
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Moves the cursor left by one character.
    ///
    /// If at the beginning of a line, moves to the end of the previous line.
    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].chars().count();
        }
    }

    /// Moves the cursor right by one character.
    ///
    /// If at the end of a line, moves to the beginning of the next line.
    pub fn move_cursor_right(&mut self) {
        let line_len = self.lines[self.cursor_line].chars().count();

        if self.cursor_col < line_len {
            self.cursor_col += 1;
        } else if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }

    /// Moves the cursor up by one line.
    ///
    /// Tries to maintain the same column position, clamping to line length.
    pub fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_len = self.lines[self.cursor_line].chars().count();
            self.cursor_col = self.cursor_col.min(line_len);
        }
    }

    /// Moves the cursor down by one line.
    ///
    /// Tries to maintain the same column position, clamping to line length.
    pub fn move_cursor_down(&mut self) {
        if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            let line_len = self.lines[self.cursor_line].chars().count();
            self.cursor_col = self.cursor_col.min(line_len);
        }
    }

    /// Moves the cursor to the beginning of the current line.
    pub fn move_cursor_home(&mut self) {
        self.cursor_col = 0;
    }

    /// Moves the cursor to the end of the current line.
    pub fn move_cursor_end(&mut self) {
        self.cursor_col = self.lines[self.cursor_line].chars().count();
    }

    /// Moves the cursor to the very start of the text (line 0, col 0).
    pub fn move_cursor_to_start(&mut self) {
        self.cursor_line = 0;
        self.cursor_col = 0;
    }

    /// Moves the cursor to the very end of the text.
    pub fn move_cursor_to_end(&mut self) {
        self.cursor_line = self.lines.len() - 1;
        self.cursor_col = self.lines[self.cursor_line].chars().count();
    }

    /// Checks if the cursor is at the very start of the text (line 0, col 0).
    ///
    /// # Returns
    ///
    /// `true` if cursor is at position (0, 0).
    pub fn is_cursor_at_start(&self) -> bool {
        self.cursor_line == 0 && self.cursor_col == 0
    }

    /// Checks if the input is empty.
    ///
    /// # Returns
    ///
    /// `true` if there's only one empty line.
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }

    /// Gets the total line count.
    ///
    /// # Returns
    ///
    /// The number of lines.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Gets the total character count (excluding newlines).
    ///
    /// # Returns
    ///
    /// The sum of characters across all lines.
    pub fn char_count(&self) -> usize {
        self.lines.iter().map(|l| l.chars().count()).sum()
    }

    /// Adjusts scroll offset to ensure cursor is visible.
    ///
    /// # Arguments
    ///
    /// * `visible_lines` - Number of lines that can be displayed
    pub fn ensure_cursor_visible(&mut self, visible_lines: usize) {
        if visible_lines == 0 {
            return;
        }

        if self.cursor_line < self.scroll_offset {
            self.scroll_offset = self.cursor_line;
        } else if self.cursor_line >= self.scroll_offset + visible_lines {
            self.scroll_offset = self.cursor_line - visible_lines + 1;
        }
    }

    /// Sets the scroll offset directly.
    ///
    /// # Arguments
    ///
    /// * `offset` - The new scroll offset
    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset.min(self.lines.len().saturating_sub(1));
    }

    /// Scrolls up by one line if possible.
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Scrolls down by one line if possible.
    ///
    /// # Arguments
    ///
    /// * `visible_lines` - Number of visible lines to prevent over-scrolling
    pub fn scroll_down(&mut self, visible_lines: usize) {
        let max_offset = self.lines.len().saturating_sub(visible_lines);
        if self.scroll_offset < max_offset {
            self.scroll_offset += 1;
        }
    }
}

impl Default for MultilineTextInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = MultilineTextInput::new();
        assert!(input.is_empty());
        assert_eq!(input.line_count(), 1);
        assert_eq!(input.cursor_pos(), (0, 0));
    }

    #[test]
    fn test_insert_char() {
        let mut input = MultilineTextInput::new();
        input.insert_char('h');
        input.insert_char('i');
        assert_eq!(input.get_text(), "hi");
        assert_eq!(input.cursor_pos(), (0, 2));
    }

    #[test]
    fn test_insert_newline() {
        let mut input = MultilineTextInput::new();
        input.insert_char('a');
        input.insert_newline();
        input.insert_char('b');
        assert_eq!(input.get_text(), "a\nb");
        assert_eq!(input.line_count(), 2);
        assert_eq!(input.cursor_pos(), (1, 1));
    }

    #[test]
    fn test_insert_newline_middle() {
        let mut input = MultilineTextInput::new();
        input.set_text("hello");
        input.move_cursor_home();
        input.move_cursor_right();
        input.move_cursor_right(); // cursor at 'l'
        input.insert_newline();
        assert_eq!(input.get_text(), "he\nllo");
        assert_eq!(input.cursor_pos(), (1, 0));
    }

    #[test]
    fn test_backspace() {
        let mut input = MultilineTextInput::new();
        input.set_text("abc");
        input.backspace();
        assert_eq!(input.get_text(), "ab");
    }

    #[test]
    fn test_backspace_join_lines() {
        let mut input = MultilineTextInput::new();
        input.set_text("a\nb");
        input.move_cursor_home(); // cursor at start of line 1
        input.backspace();
        assert_eq!(input.get_text(), "ab");
        assert_eq!(input.line_count(), 1);
    }

    #[test]
    fn test_delete() {
        let mut input = MultilineTextInput::new();
        input.set_text("abc");
        input.move_cursor_home();
        input.delete();
        assert_eq!(input.get_text(), "bc");
    }

    #[test]
    fn test_delete_join_lines() {
        let mut input = MultilineTextInput::new();
        input.set_text("a\nb");
        input.move_cursor_to_start();
        input.move_cursor_end(); // end of first line
        input.delete();
        assert_eq!(input.get_text(), "ab");
        assert_eq!(input.line_count(), 1);
    }

    #[test]
    fn test_move_cursor_up_down() {
        let mut input = MultilineTextInput::new();
        input.set_text("abc\ndef\nghi");
        input.move_cursor_to_start();

        input.move_cursor_down();
        assert_eq!(input.cursor_pos(), (1, 0));

        input.move_cursor_down();
        assert_eq!(input.cursor_pos(), (2, 0));

        input.move_cursor_up();
        assert_eq!(input.cursor_pos(), (1, 0));
    }

    #[test]
    fn test_cursor_column_clamp() {
        let mut input = MultilineTextInput::new();
        input.set_text("long line\nhi");
        input.move_cursor_to_start();
        input.move_cursor_end(); // col 9

        input.move_cursor_down();
        assert_eq!(input.cursor_pos(), (1, 2)); // clamped to line length
    }

    #[test]
    fn test_set_text_empty() {
        let mut input = MultilineTextInput::new();
        input.set_text("");
        assert!(input.is_empty());
        assert_eq!(input.line_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut input = MultilineTextInput::new();
        input.set_text("multi\nline\ntext");
        input.clear();
        assert!(input.is_empty());
        assert_eq!(input.cursor_pos(), (0, 0));
    }

    #[test]
    fn test_unicode() {
        let mut input = MultilineTextInput::new();
        input.insert_char('世');
        input.insert_char('界');
        assert_eq!(input.get_text(), "世界");
        assert_eq!(input.cursor_pos(), (0, 2));
        assert_eq!(input.char_count(), 2);
    }

    #[test]
    fn test_scroll() {
        let mut input = MultilineTextInput::new();
        input.set_text("1\n2\n3\n4\n5");

        input.scroll_down(3);
        assert_eq!(input.scroll_offset(), 1);

        input.scroll_up();
        assert_eq!(input.scroll_offset(), 0);
    }

    #[test]
    fn test_ensure_cursor_visible() {
        let mut input = MultilineTextInput::new();
        input.set_text("1\n2\n3\n4\n5\n6\n7\n8\n9\n10");
        input.move_cursor_to_end();

        input.ensure_cursor_visible(3);
        assert!(input.cursor_line >= input.scroll_offset);
        assert!(input.cursor_line < input.scroll_offset + 3);
    }
}