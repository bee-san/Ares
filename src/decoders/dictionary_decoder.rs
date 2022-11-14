use std::collections::HashMap;

use log::trace;

///! Given a dictionary such as morse code or caesar
///! a mapping of X -> Y, decode that text.
pub fn dictionary_decode(input: &Vec<&str>, dictionary: &HashMap<&str, &str>) -> Option<String> {
    let mut output_str: String = String::new();

    for x in input {
        let x = x.to_uppercase().to_string();
        // TODO support uppercase
        let Some(val) = dictionary.get(&*x) else {
            trace!("Character not found in dictionary, returning earlier {:?}", x);
            return None;
        };

        output_str.push_str(val);
    }

    Some(output_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_decode() {
        let input = vec!(".", ".", ".", ".", "-");
        let mut dictionary = HashMap::new();
        dictionary.insert(".", "e");
        dictionary.insert("-", "k");
        let output = dictionary_decode(&input, &dictionary);
        assert_eq!(output.unwrap(), "eeeek");
    }

    #[test]
    fn test_dictionary_decode_invalid() {
        let input = vec!(".");
        let dictionary = HashMap::new();
        let output = dictionary_decode(&input, &dictionary);
        assert!(output.is_none());
    }

    // Emojis are not supported
    // TODO we should support them at some point
    #[ignore]
    #[test]
    fn test_dictionary_decode_advanced_chars() {
        let input = vec!("...- /👍🏻");
        let mut dictionary = HashMap::new();
        dictionary.insert("-", "k");
        dictionary.insert(".", "a");
        dictionary.insert(" ", "x");
        dictionary.insert("/", " ");
        dictionary.insert("b", "z");
        dictionary.insert("👍🏻", "p");
        let output = dictionary_decode(&input, &dictionary);
        assert_eq!(output.unwrap(), "aaakx p");
    }
}
