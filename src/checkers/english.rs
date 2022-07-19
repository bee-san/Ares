// import storage
use crate::storage;

// given an input, check every item in the array and return true if any of them match
pub fn check_english(input: &str) -> Option<&str> {
    if let Some(result) = storage::DICTIONARIES
        .iter()
        .find(|(_, words)| words.contains(input))
    {
        // result.0 is filename
        return Some(result.0);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::checkers::english::check_english;

    #[test]
    fn test_check_and() {
        assert!(check_english("preinterview").is_some());
    }
}
