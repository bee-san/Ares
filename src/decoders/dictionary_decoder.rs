use std::{collections::HashMap, error::Error};

///! Given a dictionary such as morse code or caesar
///! a mapping of X -> Y, decode that text.


pub fn dictionary_decode(input: &str, dictionary: &HashMap<&str, &str>) -> Option<String> {
    let mut outputStr = Vec::new();
    for x in input.chars() {
        let x = x.to_string().to_uppercase().to_owned();
        // TODO support uppercase
        if !dictionary.contains_key::<str>(&x) {
            return None;
        }
        outputStr.push(dictionary[&x]);

    }
    return Some(outputStr.join(" "));
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
