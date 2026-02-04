//! Loading spinner and cryptography quotes for the TUI.
//!
//! This module provides a spinner animation with braille characters and
//! a collection of funny/interesting cryptography-related quotes to display
//! during decoding operations. Quotes are randomized each time to keep users
//! entertained with diverse cryptography facts.

use rand::seq::SliceRandom;

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
    "British triumph: Bletchley Park broke the Enigma cipher, helping win WWII!",
    "Alan Turing: British mathematician who cracked Enigma and invented the Turing machine.",
    "Fun fact: The Bombe machine at Bletchley Park tested 17,576 rotor combinations per second!",
    "British history: The Government Code and Cypher School operated from 1919, one of the oldest crypto agencies.",
    "Did you know? Over 10,000 people worked at Bletchley Park during WWII, mostly kept secret for decades!",
    "Colossus at Bletchley Park (1943): One of the first programmable computers, used to break the Lorenz cipher.",
    "Fun fact: The British 'Playfair cipher' (1854) was used in military communications for over 100 years!",
    "Trivia: The word 'decryption' became common after British codebreakers needed a verb for their work!",
    "British legend: Tommy Flowers designed Colossus, a computer that predates most other electronic computers.",
    "GCHQ secret: British researchers (Ellis, Cocks, Williamson) invented public-key cryptography in the 1970s, decades before it was publicly credited to Diffie, Hellman & Merkle. Kept classified until 1997!",
    "Clifford Cocks at GCHQ (1973) discovered RSA encryption using prime factorization—years before Rivest, Shamir & Adleman independently invented it in 1977.",
    "Fun fact: Public-key cryptography's true potential wasn't realized until the Web was invented. Tim Berners-Lee designed the open internet at CERN (1989), enabling the crypto revolution!",
    "The first recorded computer virus, 'Creeper' (1971), left the message: 'I'M THE CREEPER, CATCH ME IF YOU CAN!'",
    "In 1988, the Morris Worm crashed about 10% of the internet in a single day. The creator claimed it was a prank.",
    "Your password 'Password123!' is weak. A modern computer can crack it in seconds.",
    "The 2013 Target breach exposed 40 million credit card numbers through an air conditioning company's login.",
    "SQL Injection attacks allow hackers to access entire databases with a simple quote mark.",
    "Yahoo's 2013 breach exposed 3 billion user accounts. They delayed notification for three years.",
    "The Equifax breach (2017) exposed 147 million people's social security numbers.",
    "Phishing emails fool about 12% of recipients into handing over passwords.",
    "Default passwords like 'admin/admin' were found on 68,000 IoT devices connected to the internet.",
    "The Heartbleed bug (2014) allowed hackers to steal data from millions of computers without leaving traces.",
    "Bitcoin exchanges have been hacked for billions of dollars. One hacker stole 650,000 bitcoins.",
    "NotPetya ransomware (2017) caused an estimated $10 billion in damages globally.",
    "Ransomware operators now earn billions annually. Crime is surprisingly profitable.",
    "Two-factor authentication can be bypassed through social engineering. Hackers just call and ask.",
];

/// A loading spinner with cryptography quotes.
///
/// The spinner displays animated braille characters and cycles through
/// a collection of quotes (in randomized order) during long-running operations.
pub struct Spinner {
    /// Current spinner frame index.
    frame: usize,
    /// Current quote index in the randomized order.
    quote_index: usize,
    /// Randomized order of quote indices.
    quote_order: Vec<usize>,
}

impl Spinner {
    /// Creates a new spinner starting at the first frame and a random quote order.
    ///
    /// The quotes are shuffled into a random order each time a new spinner is created,
    /// ensuring variety across different decoding sessions.
    ///
    /// # Examples
    ///
    /// ```
    /// use ciphey::tui::spinner::Spinner;
    ///
    /// let spinner = Spinner::new();
    /// ```
    pub fn new() -> Self {
        let mut quote_order: Vec<usize> = (0..QUOTES.len()).collect();
        quote_order.shuffle(&mut rand::thread_rng());

        Self {
            frame: 0,
            quote_index: 0,
            quote_order,
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

    /// Returns the current quote from the randomized quote sequence.
    pub fn current_quote(&self) -> &'static str {
        let quote_idx = self.quote_order[self.quote_index];
        QUOTES[quote_idx]
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
        assert_eq!(spinner.quote_order.len(), QUOTES.len());
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
        assert!(Spinner::quote_count() >= 35);
        assert!(Spinner::quote_count() <= 45);
    }

    #[test]
    fn test_default() {
        let spinner = Spinner::default();
        assert_eq!(spinner.frame, 0);
        assert_eq!(spinner.quote_index, 0);
        assert_eq!(spinner.quote_order.len(), QUOTES.len());
    }
}
