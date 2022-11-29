use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

///! Morse Code Decoder
///! Does not support decoding of morse code with / instead of a space
///! or new lines for new words.
pub struct MorseCodeDecoder;

impl Crack for Decoder<MorseCodeDecoder> {
    fn new() -> Decoder<MorseCodeDecoder> {
        Decoder {
            name: "Morse Code",
            description: "Morse code is a method used in telecommunication to encode text characters as standardized sequences of two different signal durations, called dots and dashes, or dits and dahs.",
            link: "https://en.wikipedia.org/wiki/Morse_code",
            tags: vec!["morseCode", "decoder", "signals"],
            expected_runtime: 0.01,
            expected_success: 1.0,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 0.5,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Morse Code with text {:?}", text);
        // TODO support new line and slash morse code
        let decoded_text: Option<String> = text.split(' ').map(morse_to_alphanumeric).collect();

        trace!("Decoded text for morse code: {:?}", decoded_text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode Morse Code because a character was not in the dictionary");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode morse code because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }
}

/// Maps morse code to its alphanumeric character, returns None for invalid morse-code
fn morse_to_alphanumeric(text: &str) -> Option<&str> {
    let result = match text {
        ".-" => "A",
        "-..." => "B",
        "-.-." => "C",
        "-.." => "D",
        "." => "E",
        "..-." => "F",
        "--." => "G",
        "...." => "H",
        ".." => "I",
        ".---" => "J",
        "-.-" => "K",
        ".-.." => "L",
        "--" => "M",
        "-." => "N",
        "---" => "O",
        ".--." => "P",
        "--.-" => "Q",
        ".-." => "R",
        "..." => "S",
        "-" => "T",
        "..-" => "U",
        "...-" => "V",
        ".--" => "W",
        "-..-" => "X",
        "-.--" => "Y",
        "--.." => "Z",

        ".----" => "1",
        "..---" => "2",
        "...--" => "3",
        "....-" => "4",
        "....." => "5",
        "-...." => "6",
        "--..." => "7",
        "---.." => "8",
        "----." => "9",
        "-----" => "0",

        ".-..." => "&",
        ".--.-." => "@",
        "---..." => ":",
        "--..--" => ",",
        ".-.-.-" => ".",
        ".----." => "'",
        ".-..-." => "\"",
        "..--.." => "?",
        "-..-." => "/",
        "-...-" => "=",
        ".-.-." => "+",
        "-....-" => "-",
        "-.--." => "(",
        "-.--.-" => ")",
        "/" => " ",
        "-.-.--" => "!",
        " " => " ",
        "" => "",
        _ => return None,
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::athena::Athena;
    use crate::checkers::checker_type::{Check, Checker};
    use crate::checkers::CheckerTypes;
    use crate::decoders::interface::Crack;

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn test_morse_code() {
        let decoder = Decoder::<MorseCodeDecoder>::new();
        let result = decoder.crack(
            ".---- ----. ..--- .-.-.- .---- -.... ---.. .-.-.- ----- .-.-.- .----",
            &get_athena_checker(),
        );
        assert_eq!(result.unencrypted_text.unwrap()[0], "192.168.0.1");
    }
}
