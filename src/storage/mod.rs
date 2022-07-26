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
        let content = entry.contents_utf8().unwrap();
        let hash_set: HashSet<&str> = content.split_ascii_whitespace().collect();

        let filename = entry.path().to_str().unwrap();

        entries.insert(filename, hash_set);
    }
    entries
});
