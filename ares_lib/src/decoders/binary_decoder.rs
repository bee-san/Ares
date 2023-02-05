use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

///! Binary Decoder
pub struct BinaryDecoder;

impl Crack for Decoder<BinaryDecoder> {
    fn new() -> Decoder<BinaryDecoder> {
        Decoder {
            name: "Binary",
            description: "A binary code represents text, computer processor instructions, or any other data using a two-symbol system. The two-symbol system used is often \"0\" and \"1\" from the binary number system. The binary code assigns a pattern of binary digits, also known as bits, to each character, instruction, etc.",
            link: "https://en.wikipedia.org/wiki/Binary_code",
            tags: vec!["binary", "base", "decoder"],
            popularity: 1.0,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying binary with text {:?}", text);
        let mut results = CrackResult::new(self, text.to_string());
        let mut decoded_strings = Vec::new();

        for shift in 1..25 {
            let decoded_text = binary_to_string(text, shift);

            let decoded_text = decoded_text;
            decoded_strings.push(decoded_text);
            let borrowed_decoded_text = &decoded_strings[decoded_strings.len() - 1];
            if !check_string_success(borrowed_decoded_text, text) {
                debug!(
                    "Failed to decode binary because binary returned false on string {}. This means the string is 'funny' as it wasn't modified.",
                    borrowed_decoded_text
                );
                return results;
            }
            let checker_result = checker.check(borrowed_decoded_text);
            // If checkers return true, exit early with the correct result
            if checker_result.is_identified {
                info!("Found a match with binary bit {}", shift);
                results.unencrypted_text = Some(vec![borrowed_decoded_text.to_string()]);
                results.update_checker(&checker_result);
                return results;
            }
        }
        results.unencrypted_text = Some(decoded_strings);
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
}

/// Decodes binary to string
/// bit is the byte length
fn binary_to_string(binary: &str, bit: u8) -> String {
    let mut out = String::new();
    let mut iter = binary.as_bytes().iter().filter_map(|byte| match byte {
        b'0' => Some(0),
        b'1' => Some(1),
        _ => None,
    });
    loop {
        let byte = iter
            .by_ref()
            .take(usize::from(bit))
            .reduce(|acc, elem| (acc << 1) | elem);
        match byte {
            Some(byte) => out.push(char::from(byte)),
            None => break,
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::BinaryDecoder;
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
    fn binary_bit_7_decodes_successfully() {
        // This tests if Binary can decode Binary bit 7 successfully
        let decoder = Decoder::<BinaryDecoder>::new();
        let result = decoder.crack("1010011111000011010001101001110111011110000100000110111111001100100000110001011011001100001110001111010110100000111000111101011100001111001011101001111010010110001000001101010111010111001001100111110010101000001101101111100101000001110110110111111101110101110", &get_athena_checker());
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "Sphinx of black quartz, judge my vow."
        );
    }

    #[test]
    fn binary_bit_8_decodes_successfully() {
        // This tests if Binary can decode Binary bit 8 successfully
        let decoder = Decoder::<BinaryDecoder>::new();
        let result = decoder.crack("0110100001100101011011000110110001101111001000000111011101101111011100100110110001100100", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "hello world");
    }

    #[test]
    fn binary_bit_12_with_delimiters_decodes_successfully() {
        // This tests if Binary can decode Binary bit 12 with delimiters successfully
        let decoder = Decoder::<BinaryDecoder>::new();
        let result = decoder.crack("000001101000;000001110100;000001110100;000001110000;000001110011;000000111010;000000101111;000000101111;000001110111;000001110111;000001110111;000000101110;000001100111;000001101111;000001101111;000001100111;000001101100;000001100101;000000101110;000001100011;000001101111;000001101101", &get_athena_checker());
        assert_eq!(
            result.unencrypted_text.unwrap()[0],
            "https://www.google.com"
        );
    }

    #[test]
    fn binary_bit_15_with_a_lot_of_delimiters_decodes_successfully() {
        // This tests if Binary can decode Binary bit 15 with a lot of delimiters successfully
        let decoder = Decoder::<BinaryDecoder>::new();
        let result = decoder.crack(r"000+00\0001\010||100;00[000]00{}011'010'00;0'000:000:0110:10;01;0.00.000.001.11.00.11;000 ,000,000,1 00,000;0$00 0$000 0$1101$001;0!00 !00000!1 1100!1 1;000`000`000`100~000;00~000-00=0110_0011;00\\000\00\/011/0111/1 ;00?000<>000}110{11150;09008goodluck003005011h10110;00,00m00b0011f0s11f11;0h00j0r00c001t1011*00;00* 000%00011#101301;0070040 08001-1101=00;000_0 0.0001,100 .101;00090006 00113001 ~00;00d00-0 000-0101=110", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "This is convoluted.");
    }

    #[test]
    fn binary_handles_panics() {
        // This tests if Binary can handle panics
        // It should return None
        let binary_decoder = Decoder::<BinaryDecoder>::new();
        let result = binary_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn binary_handles_panic_if_empty_string() {
        // This tests if Binary can handle an empty string
        // It should return None
        let citrix_ctx1_decoder = Decoder::<BinaryDecoder>::new();
        let result = citrix_ctx1_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn binary_handles_panic_if_emoji() {
        // This tests if Binary can handle an emoji
        // It should return None
        let base64_url_decoder = Decoder::<BinaryDecoder>::new();
        let result = base64_url_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
