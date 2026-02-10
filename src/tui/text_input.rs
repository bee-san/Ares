//! Reusable text input component for TUI.
//!
//! This module provides a `TextInput` struct that encapsulates common text editing
//! functionality, eliminating duplicated code across Settings, ListEditor, and WordlistManager states.
//!
//! The [`char_to_byte_pos`] helper is shared with [`super::multiline_text_input`]
//! to avoid duplicating the char-index-to-byte-position calculation.

/// Converts a character index to a byte position within a string.
///
/// This is needed because Rust strings are UTF-8 encoded, so character positions
/// don't correspond 1:1 to byte positions for multi-byte characters.
///
/// # Arguments
///
/// * `s` - The string to index into
/// * `char_idx` - The character index to convert
///
/// # Returns
///
/// The byte position of the character at `char_idx`, or `s.len()` if
/// `char_idx` is at or past the end of the string.
pub fn char_to_byte_pos(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(pos, _)| pos)
        .unwrap_or(s.len())
}

/// A reusable text input field with cursor management.
///
/// This struct handles:
/// - Character insertion at cursor position
/// - Backspace/delete operations
/// - Cursor movement
/// - Optional maximum length
/// - Text retrieval and modification
#[derive(Debug, Clone, PartialEq)]
pub struct TextInput {
    /// The current text buffer.
    buffer: String,
    /// The current cursor position (in characters, not bytes).
    cursor_pos: usize,
    /// Optional maximum length for the input (in characters).
    max_length: Option<usize>,
}

impl TextInput {
    /// Creates a new empty `TextInput`.
    ///
    /// # Returns
    ///
    /// A new `TextInput` with an empty buffer and cursor at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let input = TextInput::new();
    /// assert_eq!(input.get_text(), "");
    /// assert_eq!(input.cursor_pos(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            cursor_pos: 0,
            max_length: None,
        }
    }

    /// Creates a new `TextInput` with a maximum length constraint.
    ///
    /// # Arguments
    ///
    /// * `max_length` - Maximum number of characters allowed
    ///
    /// # Returns
    ///
    /// A new `TextInput` with the specified maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::with_max_length(5);
    /// input.insert_char('h');
    /// input.insert_char('e');
    /// input.insert_char('l');
    /// input.insert_char('l');
    /// input.insert_char('o');
    /// input.insert_char('!'); // This won't be inserted (would exceed max)
    /// assert_eq!(input.get_text(), "hello");
    /// ```
    pub fn with_max_length(max_length: usize) -> Self {
        Self {
            buffer: String::new(),
            cursor_pos: 0,
            max_length: Some(max_length),
        }
    }

    /// Inserts a character at the current cursor position.
    ///
    /// The cursor advances by one position after insertion.
    /// If a maximum length is set and inserting would exceed it, the character is not inserted.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to insert
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// input.insert_char('b');
    /// assert_eq!(input.get_text(), "ab");
    /// assert_eq!(input.cursor_pos(), 2);
    /// ```
    pub fn insert_char(&mut self, c: char) {
        // Check max length constraint
        if let Some(max) = self.max_length {
            if self.buffer.chars().count() >= max {
                return;
            }
        }

        // Insert at cursor position (using char indices, not byte indices)
        let byte_pos = char_to_byte_pos(&self.buffer, self.cursor_pos);
        self.buffer.insert(byte_pos, c);
        self.cursor_pos += 1;
    }

    /// Deletes the character before the cursor (backspace).
    ///
    /// The cursor moves back by one position after deletion.
    /// Does nothing if the cursor is at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// input.insert_char('b');
    /// input.insert_char('c');
    /// input.backspace();
    /// assert_eq!(input.get_text(), "ab");
    /// assert_eq!(input.cursor_pos(), 2);
    /// ```
    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            let byte_pos = char_to_byte_pos(&self.buffer, self.cursor_pos);
            self.buffer.remove(byte_pos);
        }
    }

    /// Deletes the character at the cursor (delete key).
    ///
    /// The cursor position remains unchanged.
    /// Does nothing if the cursor is at the end of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// input.insert_char('b');
    /// input.insert_char('c');
    /// input.move_cursor_left();
    /// input.move_cursor_left();
    /// input.delete();
    /// assert_eq!(input.get_text(), "ac");
    /// assert_eq!(input.cursor_pos(), 1);
    /// ```
    pub fn delete(&mut self) {
        if self.cursor_pos < self.buffer.chars().count() {
            let byte_pos = char_to_byte_pos(&self.buffer, self.cursor_pos);
            self.buffer.remove(byte_pos);
        }
    }

    /// Clears the entire buffer and resets the cursor to position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// input.insert_char('b');
    /// input.clear();
    /// assert_eq!(input.get_text(), "");
    /// assert_eq!(input.cursor_pos(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor_pos = 0;
    }

    /// Sets the text and moves cursor to the end.
    ///
    /// If a maximum length is set and the text exceeds it, only the first `max_length`
    /// characters are kept.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to set
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.set_text("hello");
    /// assert_eq!(input.get_text(), "hello");
    /// assert_eq!(input.cursor_pos(), 5);
    /// ```
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.buffer = text.into();

        // Enforce max length if set
        if let Some(max) = self.max_length {
            let char_count = self.buffer.chars().count();
            if char_count > max {
                self.buffer = self.buffer.chars().take(max).collect();
            }
        }

        self.cursor_pos = self.buffer.chars().count();
    }

    /// Gets the current text as a string slice.
    ///
    /// # Returns
    ///
    /// A reference to the current text buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('h');
    /// input.insert_char('i');
    /// assert_eq!(input.get_text(), "hi");
    /// ```
    pub fn get_text(&self) -> &str {
        &self.buffer
    }

    /// Gets the current cursor position (in characters).
    ///
    /// # Returns
    ///
    /// The cursor position as a character index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// assert_eq!(input.cursor_pos(), 1);
    /// ```
    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    /// Moves the cursor one position to the left.
    ///
    /// Does nothing if the cursor is already at position 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// input.insert_char('b');
    /// input.move_cursor_left();
    /// assert_eq!(input.cursor_pos(), 1);
    /// ```
    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    /// Moves the cursor one position to the right.
    ///
    /// Does nothing if the cursor is already at the end of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('a');
    /// input.insert_char('b');
    /// input.move_cursor_left();
    /// input.move_cursor_right();
    /// assert_eq!(input.cursor_pos(), 2);
    /// ```
    pub fn move_cursor_right(&mut self) {
        let char_count = self.buffer.chars().count();
        if self.cursor_pos < char_count {
            self.cursor_pos += 1;
        }
    }

    /// Moves the cursor to the start of the input (position 0).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('h');
    /// input.insert_char('e');
    /// input.insert_char('l');
    /// input.move_cursor_home();
    /// assert_eq!(input.cursor_pos(), 0);
    /// ```
    pub fn move_cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    /// Moves the cursor to the end of the input.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('h');
    /// input.insert_char('e');
    /// input.move_cursor_home();
    /// input.move_cursor_end();
    /// assert_eq!(input.cursor_pos(), 2);
    /// ```
    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = self.buffer.chars().count();
    }

    /// Checks if the input is empty.
    ///
    /// # Returns
    ///
    /// `true` if the buffer is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let input = TextInput::new();
    /// assert!(input.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Gets the length of the input (in characters, not bytes).
    ///
    /// # Returns
    ///
    /// The number of characters in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ciphey::tui::text_input::TextInput;
    /// let mut input = TextInput::new();
    /// input.insert_char('h');
    /// input.insert_char('i');
    /// assert_eq!(input.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.buffer.chars().count()
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_text_input() {
        let input = TextInput::new();
        assert_eq!(input.get_text(), "");
        assert_eq!(input.cursor_pos(), 0);
        assert!(input.is_empty());
        assert_eq!(input.len(), 0);
    }

    #[test]
    fn test_insert_char() {
        let mut input = TextInput::new();
        input.insert_char('h');
        input.insert_char('e');
        input.insert_char('l');
        input.insert_char('l');
        input.insert_char('o');

        assert_eq!(input.get_text(), "hello");
        assert_eq!(input.cursor_pos(), 5);
        assert_eq!(input.len(), 5);
        assert!(!input.is_empty());
    }

    #[test]
    fn test_insert_unicode() {
        let mut input = TextInput::new();
        input.insert_char('世');
        input.insert_char('界');

        assert_eq!(input.get_text(), "世界");
        assert_eq!(input.cursor_pos(), 2);
        assert_eq!(input.len(), 2);
    }

    #[test]
    fn test_backspace() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');
        input.backspace();

        assert_eq!(input.get_text(), "ab");
        assert_eq!(input.cursor_pos(), 2);
    }

    #[test]
    fn test_backspace_at_start() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.move_cursor_left();
        input.backspace();

        assert_eq!(input.get_text(), "a");
        assert_eq!(input.cursor_pos(), 0);
    }

    #[test]
    fn test_delete() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');
        input.move_cursor_left();
        input.move_cursor_left();
        input.delete();

        assert_eq!(input.get_text(), "ac");
        assert_eq!(input.cursor_pos(), 1);
    }

    #[test]
    fn test_delete_at_end() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.delete();

        assert_eq!(input.get_text(), "a");
        assert_eq!(input.cursor_pos(), 1);
    }

    #[test]
    fn test_clear() {
        let mut input = TextInput::new();
        input.insert_char('h');
        input.insert_char('i');
        input.clear();

        assert_eq!(input.get_text(), "");
        assert_eq!(input.cursor_pos(), 0);
        assert!(input.is_empty());
    }

    #[test]
    fn test_set_text() {
        let mut input = TextInput::new();
        input.set_text("hello world");

        assert_eq!(input.get_text(), "hello world");
        assert_eq!(input.cursor_pos(), 11);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.insert_char('b');
        input.move_cursor_left();

        assert_eq!(input.cursor_pos(), 1);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.insert_char('b');
        input.move_cursor_left();
        input.move_cursor_right();

        assert_eq!(input.cursor_pos(), 2);
    }

    #[test]
    fn test_move_cursor_home() {
        let mut input = TextInput::new();
        input.insert_char('h');
        input.insert_char('e');
        input.insert_char('l');
        input.move_cursor_home();

        assert_eq!(input.cursor_pos(), 0);
    }

    #[test]
    fn test_move_cursor_end() {
        let mut input = TextInput::new();
        input.insert_char('h');
        input.insert_char('e');
        input.move_cursor_home();
        input.move_cursor_end();

        assert_eq!(input.cursor_pos(), 2);
    }

    #[test]
    fn test_insert_at_middle() {
        let mut input = TextInput::new();
        input.insert_char('a');
        input.insert_char('c');
        input.move_cursor_left();
        input.insert_char('b');

        assert_eq!(input.get_text(), "abc");
        assert_eq!(input.cursor_pos(), 2);
    }

    #[test]
    fn test_max_length() {
        let mut input = TextInput::with_max_length(3);
        input.insert_char('a');
        input.insert_char('b');
        input.insert_char('c');
        input.insert_char('d'); // Should not be inserted

        assert_eq!(input.get_text(), "abc");
        assert_eq!(input.len(), 3);
    }

    #[test]
    fn test_set_text_with_max_length() {
        let mut input = TextInput::with_max_length(5);
        input.set_text("hello world");

        assert_eq!(input.get_text(), "hello");
        assert_eq!(input.cursor_pos(), 5);
    }

    #[test]
    fn test_default() {
        let input = TextInput::default();
        assert_eq!(input.get_text(), "");
        assert_eq!(input.cursor_pos(), 0);
    }

    #[test]
    fn test_backspace_unicode() {
        let mut input = TextInput::new();
        input.insert_char('h');
        input.insert_char('世');
        input.insert_char('界');
        input.backspace();

        assert_eq!(input.get_text(), "h世");
        assert_eq!(input.cursor_pos(), 2);
    }
}
