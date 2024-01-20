use include_dir::include_dir;
use include_dir::Dir;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

/// Tells Rust to load the dictionaries into the binary
/// at compile time. Which means that we do not waste
/// time loading them at runtime.
pub static DICTIONARIES: Lazy<HashMap<&str, HashSet<&str>>> = Lazy::new(|| {
    /// The directory where our dictionaries are stored.
    static DICTIONARIES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/storage/dictionaries");
    let mut entries = HashMap::new();

    for entry in DICTIONARIES_DIR.files() {
        let content = entry.contents_utf8().expect("The file you moved into the dictionaries folder is not UTF-8. The storage module only works on UTF-8 files.");
        let hash_set: HashSet<&str> = content.split_ascii_whitespace().collect();

        let filename = entry.path().to_str().expect(
            "Cannot turn filename of the file you moved into the Dictionaries folder into a string",
        );

        entries.insert(filename, hash_set);
    }
    entries
});

/// Loads invisible character list into a HashSet
pub static INVISIBLE_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
    /// The directory where the unicode of invisible characters are stored.
    static INVIS_CHARS_DIR: Dir<'_> =
        include_dir!("$CARGO_MANIFEST_DIR/src/storage/invisible_chars");
    let mut entries: HashSet<char> = HashSet::new();
    for entry in INVIS_CHARS_DIR.files() {
        let content = entry.contents_utf8().expect(
            "The file you moved into the invisible_chars folder is not UTF-8. \
            The storage module only works on UTF-8 files.",
        );
        let content_lines = content.split('\n');
        for line in content_lines {
            if line.is_empty() {
                continue;
            }
            let unicode_line_split: Vec<&str> = line.split_ascii_whitespace().collect();
            let unicode_literal = unicode_line_split[0].trim_start_matches("U+");
            let unicode_char = u32::from_str_radix(unicode_literal, 16)
                .ok()
                .and_then(char::from_u32)
                .expect(
                    "The file you moved into the invisible_chars folder should \
                    contain valid unicode literals in the first column.",
                );
            entries.insert(unicode_char);
        }
    }
    entries
});

// Rust tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_loads_words_txt() {
        assert!(DICTIONARIES.contains_key("words.txt"))
    }
    #[test]
    fn test_dictionary_loads_words_txt_contains_hello() {
        assert!(DICTIONARIES.get("words.txt").unwrap().contains("hello"))
    }

    #[test]
    fn test_dictionary_does_not_contain_single_letter_words() {
        assert!(!DICTIONARIES.get("words.txt").unwrap().contains("a"))
    }
}
