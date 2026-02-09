//! NATO Phonetic Alphabet decoder
//! Converts NATO phonetic alphabet words back to their corresponding letters
//! Example: "Alpha Bravo Charlie" → "ABC"

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};
use std::collections::HashMap;

/// NATO Phonetic Alphabet Decoder
/// Converts NATO phonetic alphabet words (Alpha, Bravo, Charlie, etc.) back to letters
/// ```
/// use ciphey::decoders::nato_phonetic_decoder::NATOPhoneticDecoder;
/// use ciphey::decoders::interface::{Crack, Decoder};
/// use ciphey::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decoder = Decoder::<NATOPhoneticDecoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decoder.crack("Alpha Bravo Charlie", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "ABC");
/// ```
pub struct NATOPhoneticDecoder;

impl Crack for Decoder<NATOPhoneticDecoder> {
    fn new() -> Decoder<NATOPhoneticDecoder> {
        Decoder {
            name: "NATO Phonetic",
            description: "The NATO phonetic alphabet is the most widely used radiotelephone spelling alphabet. It assigns code words to the letters of the English alphabet acrophonically (Alpha for A, Bravo for B, etc.) so that critical combinations of letters and numbers can be pronounced and understood clearly.",
            link: "https://en.wikipedia.org/wiki/NATO_phonetic_alphabet",
            tags: vec!["nato", "phonetic", "decoder", "alphabet"],
            popularity: 0.5,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying NATO Phonetic with text {:?}", text);
        let decoded_text = decode_nato_phonetic(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode NATO Phonetic because decode_nato_phonetic returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode NATO Phonetic because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }

    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }

    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }

    /// Gets the description for the current decoder
    fn get_description(&self) -> &str {
        self.description
    }

    /// Gets the link for the current decoder
    fn get_link(&self) -> &str {
        self.link
    }
}

/// Build the NATO phonetic alphabet mapping (word -> character)
fn get_nato_mapping() -> HashMap<&'static str, char> {
    let mut mapping = HashMap::new();

    // Letters A-Z
    mapping.insert("alfa", 'A');
    mapping.insert("alpha", 'A'); // Common alternate spelling
    mapping.insert("bravo", 'B');
    mapping.insert("charlie", 'C');
    mapping.insert("delta", 'D');
    mapping.insert("echo", 'E');
    mapping.insert("foxtrot", 'F');
    mapping.insert("golf", 'G');
    mapping.insert("hotel", 'H');
    mapping.insert("india", 'I');
    mapping.insert("juliet", 'J');
    mapping.insert("juliett", 'J'); // Official spelling with double T
    mapping.insert("kilo", 'K');
    mapping.insert("lima", 'L');
    mapping.insert("mike", 'M');
    mapping.insert("november", 'N');
    mapping.insert("oscar", 'O');
    mapping.insert("papa", 'P');
    mapping.insert("quebec", 'Q');
    mapping.insert("romeo", 'R');
    mapping.insert("sierra", 'S');
    mapping.insert("tango", 'T');
    mapping.insert("uniform", 'U');
    mapping.insert("victor", 'V');
    mapping.insert("whiskey", 'W');
    mapping.insert("whisky", 'W'); // Alternate spelling
    mapping.insert("xray", 'X');
    mapping.insert("x-ray", 'X'); // With hyphen
    mapping.insert("yankee", 'Y');
    mapping.insert("zulu", 'Z');

    // Numbers 0-9
    mapping.insert("zero", '0');
    mapping.insert("one", '1');
    mapping.insert("two", '2');
    mapping.insert("three", '3');
    mapping.insert("four", '4');
    mapping.insert("five", '5');
    mapping.insert("six", '6');
    mapping.insert("seven", '7');
    mapping.insert("eight", '8');
    mapping.insert("nine", '9');
    mapping.insert("niner", '9'); // Aviation variant

    mapping
}

/// Decode NATO phonetic alphabet text to plain text
/// Returns None if input doesn't appear to be valid NATO phonetic
fn decode_nato_phonetic(text: &str) -> Option<String> {
    if text.is_empty() {
        return None;
    }

    let mapping = get_nato_mapping();

    // First, normalize "x-ray" to "xray" before replacing hyphens with spaces
    let text_with_xray = text.to_lowercase().replace("x-ray", "xray");

    // Normalize the text: replace various delimiters with spaces
    let normalized = text_with_xray
        .replace([',', ';', ':', '\n', '\r', '\t'], " ")
        .replace('-', " ");

    // Split by whitespace and filter empty strings
    let words: Vec<&str> = normalized.split_whitespace().collect();

    if words.is_empty() {
        return None;
    }

    let mut result = String::new();
    let mut found_count = 0;

    for word in &words {
        if let Some(&c) = mapping.get(*word) {
            result.push(c);
            found_count += 1;
        }
        // Silently skip unrecognized words (could be noise/separators)
    }

    // Only consider it a valid NATO decode if we found at least some NATO words
    // and a reasonable percentage of the input words were recognized
    if found_count == 0 {
        return None;
    }

    // Require at least 50% of words to be recognized NATO words
    let recognition_ratio = found_count as f64 / words.len() as f64;
    if recognition_ratio < 0.5 {
        return None;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::NATOPhoneticDecoder;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn nato_decodes_abc_successfully() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Alpha Bravo Charlie", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABC");
    }

    #[test]
    fn nato_decodes_hello_successfully() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Hotel Echo Lima Lima Oscar", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLO");
    }

    #[test]
    fn nato_decodes_hello_world_successfully() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack(
            "Hotel Echo Lima Lima Oscar Whiskey Oscar Romeo Lima Delta",
            &get_athena_checker(),
        );
        assert_eq!(result.unencrypted_text.unwrap()[0], "HELLOWORLD");
    }

    #[test]
    fn nato_decodes_with_numbers() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Alpha One Bravo Two Charlie Three", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "A1B2C3");
    }

    #[test]
    fn nato_decodes_lowercase() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("alpha bravo charlie", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABC");
    }

    #[test]
    fn nato_decodes_uppercase() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("ALPHA BRAVO CHARLIE", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABC");
    }

    #[test]
    fn nato_decodes_mixed_case() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Alpha BRAVO charlie DELTA", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABCD");
    }

    #[test]
    fn nato_decodes_with_comma_delimiters() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Alpha, Bravo, Charlie", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABC");
    }

    #[test]
    fn nato_decodes_with_hyphen_delimiters() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Alpha-Bravo-Charlie", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABC");
    }

    #[test]
    fn nato_decodes_with_newlines() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Alpha\nBravo\nCharlie", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "ABC");
    }

    #[test]
    fn nato_decodes_alternate_spellings() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        // Test Alfa vs Alpha, Juliett vs Juliet, Whisky vs Whiskey
        let result = decoder.crack("Alfa Juliett Whisky", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "AJW");
    }

    #[test]
    fn nato_decodes_xray_with_hyphen() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("X-ray Yankee Zulu", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "XYZ");
    }

    #[test]
    fn nato_decodes_niner() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("Niner Eight Seven", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "987");
    }

    #[test]
    fn nato_decode_empty_string() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn nato_decode_invalid_input() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder
            .crack("This is not NATO phonetic", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn nato_decode_handles_panics() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn nato_handle_panic_if_whitespace_only() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder
            .crack("   \t\n  ", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn nato_handle_panic_if_emoji() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack("😂", &get_athena_checker()).unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn nato_full_alphabet() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack(
            "Alpha Bravo Charlie Delta Echo Foxtrot Golf Hotel India Juliet Kilo Lima Mike November Oscar Papa Quebec Romeo Sierra Tango Uniform Victor Whiskey Xray Yankee Zulu",
            &get_athena_checker(),
        );
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
    }

    #[test]
    fn nato_all_numbers() {
        let decoder = Decoder::<NATOPhoneticDecoder>::new();
        let result = decoder.crack(
            "Zero One Two Three Four Five Six Seven Eight Nine",
            &get_athena_checker(),
        );
        assert_eq!(result.unencrypted_text.unwrap()[0], "0123456789");
    }
}
