use std::collections::HashSet;
use once_cell::sync::Lazy;

pub static STORAGE: Lazy<HashSet<&str>> = Lazy::new(|| {
    let hashset_for_words: HashSet<&str> = include_str!("dictionaries/words.txt")
        .split_ascii_whitespace()
        .collect();

    hashset_for_words
});
