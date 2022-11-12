use std::{collections::HashMap, error::Error};

///! Given a dictionary such as morse code or caesar
///! a mapping of X -> Y, decode that text.


pub fn dictionary_decode(input: &str, dictionary: &HashMap<char, char>) -> Option<String> {
    let mut outputStr = String::new();
    for x in input.chars() {
        // TODO support uppercase
        if !dictionary.contains_key(&x) {
            return None;
        }
        outputStr.push(dictionary[&x]);

    }
    return Some(outputStr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_decode() {
        let input = "....-";
        let mut dictionary = HashMap::new();
        dictionary.insert('.', 'e');
        dictionary.insert('-', 'k');
        let output = dictionary_decode(&input, &dictionary);
        assert_eq!(output.unwrap(), "eeeek");
    }

    #[test]
    fn test_dictionary_decode_invalid() {
        let input = ".";
        let mut dictionary = HashMap::new();
        let output = dictionary_decode(&input, &dictionary);
        assert!(output.is_none());
    }
}
