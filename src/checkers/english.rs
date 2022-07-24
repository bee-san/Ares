// import storage
use crate::checkers::checker_object::CheckObject;
use crate::storage;

// given an input, check every item in the array and return true if any of them match
pub fn check_english(input: &str) -> Option<CheckObject> {
    if let Some(result) = storage::DICTIONARIES
        .iter()
        .find(|(_, words)| words.contains(input))
    {
        // result.0 is filename
        return Some(CheckObject {
            is_identified: true,
            text: input,
            checker: "Dictionary",
            description: result.0.to_string(),
            link: "https://en.wikipedia.org/wiki/List_of_English_words",
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::checkers::english::check_english;

    #[test]
    fn test_check_basic() {
        assert!(check_english("preinterview").is_some());
    }

    #[test]
    fn test_check_basic2() {
        assert!(check_english("and").is_some());
    }

    #[test]
    fn test_check_multiple_words() {
        assert!(check_english("and woody").is_none());
    }

    #[test]
    fn test_check_non_dictionary_word() {
        assert!(
            check_english("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaBabyShark").is_none()
        );
    }
    // TODO make these obvious succeed or fail cases

    #[test]
    fn test_check_rock_you() {
        assert!(
            check_english("!naruto").is_some()
        );
    }
}
