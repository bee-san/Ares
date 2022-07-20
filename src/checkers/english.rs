// import storage
use crate::storage;
use crate::checkers::checkerObject::{CheckObject};

// given an input, check every item in the array and return true if any of them match
pub fn check_english(input: &str) -> Option<CheckObject> {
    if let Some(result) = storage::DICTIONARIES
        .iter()
        .find(|(_, words)| words.contains(input))
    {
        // result.0 is filename
        return Some(CheckObject{
            is_identified: true,
            text: input.to_string(),
            checker: "Dictionary".to_string(),
            description: result.0.to_string(),
            link: "https://en.wikipedia.org/wiki/List_of_English_words".to_string(),
        });
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
