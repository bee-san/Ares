use std::collections::HashMap;

///! Given a dictionary such as morse code or caesar
///! a mapping of X -> Y, decode that text.
pub fn dictionary_decode(input: &str, dictionary: &HashMap<&str, &str>) -> Option<String> {
    let mut output_str: String = String::with_capacity(input.len());

    for x in input.chars() {
        let x = x.to_uppercase().to_string();
        // TODO support uppercase
        let Some(val) = dictionary.get(&*x) else {
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
        let input = "....-";
        let mut dictionary = HashMap::new();
        dictionary.insert(".", "e");
        dictionary.insert("-", "k");
        let output = dictionary_decode(input, &dictionary);
        assert_eq!(output.unwrap(), "eeeek");
    }

    #[test]
    fn test_dictionary_decode_invalid() {
        let input = ".";
        let dictionary = HashMap::new();
        let output = dictionary_decode(input, &dictionary);
        assert!(output.is_none());
    }
}
