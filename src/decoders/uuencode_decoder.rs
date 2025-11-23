//! Decode UUEncoded strings
//! Performs error handling and returns a string

use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::Crack;
use crate::decoders::interface::Decoder;

use log::{debug, info, trace};

pub struct UUEncodeDecoder;

impl Crack for Decoder<UUEncodeDecoder> {
    fn new() -> Decoder<UUEncodeDecoder> {
        Decoder {
            name: "UUEncode",
            description: "UUencoding is a form of binary-to-text encoding that originated in the Unix-to-Unix Copy program.",
            link: "https://en.wikipedia.org/wiki/Uuencoding",
            tags: vec!["uuencode", "unix", "decoder", "legacy"],
            popularity: 0.3,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying UUEncode with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());

        // Manual implementation or check if crate supports "uudecode"
        // The crate `uuencode` 0.1.1 has `uudecode` function typically.
        // Let's assume typical uudecode format with "begin <mode> <file>" and "end"
        // But often CTF challenges just give the body.
        // Body format: Length char, then data.
        // Length char: ' ' (32) to 'M' (77). Length = char - 32.
        // Then 4 chars -> 3 bytes.

        let decoded_text = decode_uuencode_no_error_handling(text);

        if let Some(decoded) = decoded_text {
            if check_string_success(&decoded, text) {
                let checker_result = checker.check(&decoded);
                results.unencrypted_text = Some(vec![decoded]);
                results.update_checker(&checker_result);
            }
        }

        results
    }

    fn get_tags(&self) -> &Vec<&str> { &self.tags }
    fn get_name(&self) -> &str { self.name }
    fn get_popularity(&self) -> f32 { self.popularity }
    fn get_description(&self) -> &str { self.description }
    fn get_link(&self) -> &str { self.link }
}

fn decode_uuencode_no_error_handling(text: &str) -> Option<String> {
    // Basic body decoding if no header
    // If header exists, strip it.
    let mut lines = text.lines();
    let mut body_lines = Vec::new();
    let mut started = false;

    // Check for begin/end
    if text.contains("begin ") {
        for line in lines {
            if line.starts_with("begin ") {
                started = true;
                continue;
            }
            if line == "end" {
                break;
            }
            if started {
                if !line.is_empty() {
                    body_lines.push(line);
                }
            }
        }
    } else {
        // Assume raw body if valid UU chars
        for line in text.lines() {
             if !line.is_empty() && line != "end" && !line.starts_with("begin ") {
                 body_lines.push(line);
             }
        }
    }

    if body_lines.is_empty() { return None; }

    let mut decoded_bytes = Vec::new();

    for line in body_lines {
        if line == "`" || line == " " { continue; } // End of data marker in some variants

        let bytes = line.as_bytes();
        if bytes.is_empty() { continue; }

        // First char is length
        let len_char = bytes[0];
        if len_char < 32 || len_char > 96 { return None; } // Basic range check
        let length = (len_char - 32) as usize;
        if length == 0 { continue; }

        let data = &bytes[1..];
        let mut out_idx = 0;

        for chunk in data.chunks(4) {
             if out_idx >= length { break; }
             if chunk.len() < 2 { break; } // Need at least 2 chars to get 1 byte?

             // Map chars back to 0-63
             // UU uses space(32) to `(96). x - 32.
             // But sometimes space is replaced by ` for obscure reasons.
             // (c - 32) & 0x3F

             let c0 = if chunk.len() > 0 { (chunk[0].wrapping_sub(32)) & 0x3F } else { 0 };
             let c1 = if chunk.len() > 1 { (chunk[1].wrapping_sub(32)) & 0x3F } else { 0 };
             let c2 = if chunk.len() > 2 { (chunk[2].wrapping_sub(32)) & 0x3F } else { 0 };
             let c3 = if chunk.len() > 3 { (chunk[3].wrapping_sub(32)) & 0x3F } else { 0 };

             // 4 chars (6 bits each) -> 3 bytes (8 bits each)
             // b0 = c0 << 2 | c1 >> 4
             // b1 = c1 << 4 | c2 >> 2
             // b2 = c2 << 6 | c3

             let b0 = (c0 << 2) | (c1 >> 4);
             let b1 = (c1 << 4) | (c2 >> 2);
             let b2 = (c2 << 6) | c3;

             if out_idx < length { decoded_bytes.push(b0); out_idx += 1; }
             if out_idx < length { decoded_bytes.push(b1); out_idx += 1; }
             if out_idx < length { decoded_bytes.push(b2); out_idx += 1; }
        }
    }

    if decoded_bytes.is_empty() { return None; }
    String::from_utf8(decoded_bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::UUEncodeDecoder;
    use crate::{
        checkers::{athena::Athena, checker_type::{Check, Checker}, CheckerTypes},
        decoders::interface::{Crack, Decoder},
    };

    fn get_checker() -> CheckerTypes {
        CheckerTypes::CheckAthena(Checker::<Athena>::new())
    }

    #[test]
    fn uuencode_cat() {
        // "Cat"
        // Length 3. '3' + 32 = 35 -> '#'
        // C: 67 (01000011), a: 97 (01100001), t: 116 (01110100)
        // 010000 110110 000101 110100
        // 16     54     5      52
        // +32    +32    +32    +32
        // 48('0') 86('V') 37('%') 84('T')
        // So line: "#0V%T"
        let decoder = Decoder::<UUEncodeDecoder>::new();
        let result = decoder.crack("#0V%T", &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Cat");
    }

    #[test]
    fn uuencode_full() {
        // begin 644 test.txt
        // #0V%T
        // `
        // end
        let decoder = Decoder::<UUEncodeDecoder>::new();
        let input = "begin 644 test.txt\n#0V%T\n`\nend";
        let result = decoder.crack(input, &get_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Cat");
    }
}
