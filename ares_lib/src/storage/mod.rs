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
