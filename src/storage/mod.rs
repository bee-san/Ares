use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Loads invisible character list into a HashSet
pub static INVISIBLE_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    let mut entries: HashSet<char> = HashSet::new();

    // Path to the invisible characters file
    let chars_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("storage")
        .join("invisible_chars")
        .join("chars.txt");

    // Read the file content
    if let Ok(content) = fs::read_to_string(&chars_file_path) {
        let content_lines = content.split('\n');
        for line in content_lines {
            if line.is_empty() {
                continue;
            }
            let unicode_line_split: Vec<&str> = line.split_ascii_whitespace().collect();
            if unicode_line_split.is_empty() {
                continue;
            }
            let unicode_literal = unicode_line_split[0].trim_start_matches("U+");
            if let Ok(unicode_value) = u32::from_str_radix(unicode_literal, 16) {
                if let Some(unicode_char) = char::from_u32(unicode_value) {
                    entries.insert(unicode_char);
                }
            }
        }
    }

    entries
});

// Rust tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invisible_chars_loaded() {
        // Verify that the INVISIBLE_CHARS HashSet is not empty
        assert!(!INVISIBLE_CHARS.is_empty());
    }

    #[test]
    fn test_invisible_chars_contains_space() {
        // Verify that the space character (U+0020) is in the HashSet
        assert!(INVISIBLE_CHARS.contains(&' '));
    }

    #[test]
    fn test_invisible_chars_contains_zero_width_space() {
        // Verify that the zero width space (U+200B) is in the HashSet
        // This is a common invisible character
        let zero_width_space = char::from_u32(0x200B).unwrap();
        assert!(INVISIBLE_CHARS.contains(&zero_width_space));
    }
}
