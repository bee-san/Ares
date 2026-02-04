//! Loading spinner and cryptography quotes for the TUI.
//!
//! This module provides a spinner animation with braille characters and
//! a collection of funny/interesting cryptography-related quotes to display
//! during decoding operations.

/// Braille/unicode spinner frames for smooth animation.
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Enhanced braille spinner frames for more visible animation (fuller dots).
pub const ENHANCED_SPINNER_FRAMES: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// Collection of funny and interesting cryptography-related quotes.
const QUOTES: &[&str] = &[
    "The only truly secure system is one that is powered off. - Gene Spafford",
    "Cryptography is typically bypassed, not penetrated. - Adi Shamir",
    "Base64 isn't encryption, it's encoding. Now you know!",
    "Brute forcing 26 keys since 44 BC...",
    "Fun fact: The Caesar cipher is named after Julius Caesar!",
    "Security is a process, not a product. - Bruce Schneier",
    "There are two types of encryption: one that will prevent your sister from reading your diary, and one that will prevent your government. - Bruce Schneier",
    "The Enigma machine had 158,962,555,217,826,360,000 possible settings!",
    "ROT13 twice for extra security! (Just kidding, please don't.)",
    "If you think cryptography can solve your problem, you don't understand cryptography. - Peter Neumann",
    "Fun fact: The word 'cipher' comes from the Arabic 'sifr' meaning zero!",
    "Attempting to decode... or maybe just staring at random bytes.",
    "In cryptography, we trust math, not people.",
    "The Vigenere cipher was once called 'le chiffre indechiffrable' (the unbreakable cipher). We broke it.",
];

/// A loading spinner with cryptography quotes.
///
/// The spinner displays animated braille characters and cycles through
/// a collection of quotes during long-running operations.
pub struct Spinner {
    /// Current spinner frame index.
    frame: usize,
    /// Current quote index.
    quote_index: usize,
}

impl Spinner {
    /// Creates a new spinner starting at the first frame and quote.
    ///
    /// # Examples
    ///
    /// ```
    /// use ciphey::tui::spinner::Spinner;
    ///
    /// let spinner = Spinner::new();
    /// ```
    pub fn new() -> Self {
        Self {
            frame: 0,
            quote_index: 0,
        }
    }

    /// Advances the spinner to the next frame.
    ///
    /// The frame wraps around to the beginning when it reaches the end.
    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % SPINNER_FRAMES.len();
    }

    /// Advances to the next quote.
    ///
    /// The quote wraps around to the beginning when it reaches the end.
    pub fn next_quote(&mut self) {
        self.quote_index = (self.quote_index + 1) % QUOTES.len();
    }

    /// Returns the current spinner frame character.
    pub fn current_frame(&self) -> &'static str {
        SPINNER_FRAMES[self.frame]
    }

    /// Returns the current quote.
    pub fn current_quote(&self) -> &'static str {
        QUOTES[self.quote_index]
    }

    /// Returns the total number of spinner frames.
    pub fn frame_count() -> usize {
        SPINNER_FRAMES.len()
    }

    /// Returns the total number of quotes.
    pub fn quote_count() -> usize {
        QUOTES.len()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_new() {
        let spinner = Spinner::new();
        assert_eq!(spinner.frame, 0);
        assert_eq!(spinner.quote_index, 0);
    }

    #[test]
    fn test_spinner_tick() {
        let mut spinner = Spinner::new();
        assert_eq!(spinner.current_frame(), "⠋");
        spinner.tick();
        assert_eq!(spinner.current_frame(), "⠙");
    }

    #[test]
    fn test_spinner_tick_wraps() {
        let mut spinner = Spinner::new();
        for _ in 0..SPINNER_FRAMES.len() {
            spinner.tick();
        }
        assert_eq!(spinner.frame, 0);
        assert_eq!(spinner.current_frame(), "⠋");
    }

    #[test]
    fn test_next_quote() {
        let mut spinner = Spinner::new();
        let first_quote = spinner.current_quote();
        spinner.next_quote();
        let second_quote = spinner.current_quote();
        assert_ne!(first_quote, second_quote);
    }

    #[test]
    fn test_next_quote_wraps() {
        let mut spinner = Spinner::new();
        for _ in 0..QUOTES.len() {
            spinner.next_quote();
        }
        assert_eq!(spinner.quote_index, 0);
    }

    #[test]
    fn test_frame_count() {
        assert_eq!(Spinner::frame_count(), 10);
    }

    #[test]
    fn test_quote_count() {
        assert!(Spinner::quote_count() >= 10);
        assert!(Spinner::quote_count() <= 15);
    }

    #[test]
    fn test_default() {
        let spinner = Spinner::default();
        assert_eq!(spinner.frame, 0);
        assert_eq!(spinner.quote_index, 0);
    }
}
