///! Decode a base64 string
///! Performs error handling and returns a string
///! Call base64_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

pub struct MorseCodeDecoder;

impl Crack for Decoder<Base64Decoder> {
    fn new() -> Decoder<Base64Decoder> {
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
        
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base64 because Base64Decoder::decode_base64_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base64 because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(decoded_text);

        results.update_checker(&checker_result);

        results
    }

    fn _morse_dictionary() -> HashMap<&'static char, &'static str> {
        map! {
            "A" => ".-",      "B" => "-...",    "C" => "-.-.",
            "D" => "-..",     "E" => ".",       "F" => "..-.",
            "G" => "--.",     "H" => "....",    "I" => "..",
            "J" => ".---",    "K" => "-.-",     "L" => ".-..",
            "M" => "--",      "N" => "-.",      "O" => "---",
            "P" => ".--.",    "Q" => "--.-",    "R" => ".-.",
            "S" => "...",     "T" => "-",       "U" => "..-",
            "V" => "...-",    "W" => ".--",     "X" => "-..-",
            "Y" => "-.--",    "Z" => "--..",
    
            "1" => ".----",   "2" => "..---",   "3" => "...--",
            "4" => "....-",   "5" => ".....",   "6" => "-....",
            "7" => "--...",   "8" => "---..",   "9" => "----.",
            "0" => "-----",
    
            "&" => ".-...",   "@" => ".--.-.",  ":" => "---...",
            "," => "--..--",  "." => ".-.-.-",  "'" => ".----.",
            "\"" => ".-..-.", "?" => "..--..",  "/" => "-..-.",
            "=" => "-...-",   "+" => ".-.-.",   "-" => "-....-",
            "(" => "-.--.",   ")" => "-.--.-",  " " => "/",
            "!" => "-.-.--",
        }
    }
}
