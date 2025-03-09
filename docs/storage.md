# Storage Module

The storage module provides reusable data structures and constants that are used across the ciphey project.

## Contents

### English Letter Frequencies

The `ENGLISH_FREQS` constant provides the frequency distribution of letters in the English language. This is used for frequency analysis in various decoders, such as the Vigenere decoder.

```rust
pub const ENGLISH_FREQS: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, // V-Z
];
```

These values represent the relative frequency of each letter in typical English text, from A to Z. They are used in statistical analysis for breaking classical ciphers.

### Invisible Characters

The `INVISIBLE_CHARS` static collection contains a set of invisible Unicode characters that are loaded from a file at runtime. This is used for detecting and handling invisible characters in encoded text.

```rust
pub static INVISIBLE_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    // Implementation loads characters from a file
    // ...
});
```

The characters are loaded from `src/storage/invisible_chars/chars.txt` and include various whitespace and zero-width characters.

## Usage

To use these resources in your code:

```rust
use crate::storage::ENGLISH_FREQS;
use crate::storage::INVISIBLE_CHARS;

// Example: Using English frequencies for analysis
fn analyze_text(text: &str) {
    // ...frequency analysis using ENGLISH_FREQS...
}

// Example: Checking for invisible characters
fn check_for_invisible(text: &str) -> bool {
    text.chars().any(|c| INVISIBLE_CHARS.contains(&c))
}