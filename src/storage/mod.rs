use include_dir::Dir;
use include_dir::include_dir;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

pub static DICTIONARIES: Lazy<HashMap<&str, HashSet<&str>>> = Lazy::new(|| {
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
