//! Proposal: https://broadleaf-angora-7db.notion.site/Filtration-System-7143b36a42f1466faea3077bfc7e859e
//! Given a filter object, return an array of decoders/crackers which have been filtered

use crate::checkers::CheckerTypes;
use crate::decoders::atbash_decoder::AtbashDecoder;
use crate::decoders::base32_decoder::Base32Decoder;
use crate::decoders::base58_bitcoin_decoder::Base58BitcoinDecoder;
use crate::decoders::base58_monero_decoder::Base58MoneroDecoder;
use crate::decoders::binary_decoder::BinaryDecoder;
use crate::decoders::hexadecimal_decoder::HexadecimalDecoder;
use crate::decoders::octal_decoder::OctalDecoder;
use crate::DecoderResult;

use crate::decoders::base58_flickr_decoder::Base58FlickrDecoder;
use crate::decoders::base58_ripple_decoder::Base58RippleDecoder;

use crate::decoders::a1z26_decoder::A1Z26Decoder;
use crate::decoders::ascii85_decoder::Ascii85Decoder;
use crate::decoders::base62_decoder::Base62Decoder;
use crate::decoders::base64_decoder::Base64Decoder;
use crate::decoders::base65536_decoder::Base65536Decoder;
use crate::decoders::base91_decoder::Base91Decoder;
use crate::decoders::braille_decoder::BrailleDecoder;
use crate::decoders::caesar_decoder::CaesarDecoder;
use crate::decoders::citrix_ctx1_decoder::CitrixCTX1Decoder;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::{Crack, Decoder};
use crate::decoders::morse_code::MorseCodeDecoder;
use crate::decoders::railfence_decoder::RailfenceDecoder;
use crate::decoders::reverse_decoder::ReverseDecoder;
use crate::decoders::rot47_decoder::ROT47Decoder;
use crate::decoders::substitution_generic_decoder::SubstitutionGenericDecoder;
use crate::decoders::url_decoder::URLDecoder;
use crate::decoders::vigenere_decoder::VigenereDecoder;
use crate::decoders::z85_decoder::Z85Decoder;

use crate::decoders::brainfuck_interpreter::BrainfuckInterpreter;
use crate::decoders::nato_phonetic_decoder::NATOPhoneticDecoder;

use crate::config::get_config;
use log::trace;
use rayon::prelude::*;

/// The struct which contains all of the decoders
/// Where decoders is crackers, decryptors, etc.
/// This contains a public attribute Components
/// Which contains all of them. See `pub fn run` which is impl'd on
/// the Decoders for the Crack trait in action.
/// Relevant docs: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
pub struct Decoders {
    /// Components is a vector of decoders.
    pub components: Vec<Box<dyn Crack + Sync>>,
}

/// Default number of decoders to run concurrently
const DEFAULT_DECODER_BATCH_SIZE: usize = 4;

impl Decoders {
    /// Iterate over all of the decoders and run .crack(text) on them.
    ///
    /// Unlike the previous implementation, this does NOT short-circuit on first success.
    /// Instead, it collects ALL results and returns the best successful result based on
    /// decoder popularity (higher popularity = preferred). This prevents race conditions
    /// where a false positive from a slower decoder beats the correct result.
    ///
    /// Decoders are processed in batches to limit concurrency and ensure predictable ordering.
    ///
    /// We are using Trait Objects
    /// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
    /// Which allows us to have multiple different structs in the same vector
    /// But each struct shares the same `.crack()` method, so it's fine.
    pub fn run(&self, text: &str, checker: CheckerTypes) -> MyResults {
        self.run_with_batch_size(text, checker, DEFAULT_DECODER_BATCH_SIZE)
    }

    /// Run decoders with a specific batch size for concurrency control.
    ///
    /// # Arguments
    /// * `text` - The text to decode
    /// * `checker` - The checker to validate results
    /// * `batch_size` - Maximum number of decoders to run concurrently
    pub fn run_with_batch_size(
        &self,
        text: &str,
        checker: CheckerTypes,
        batch_size: usize,
    ) -> MyResults {
        trace!(
            "Running .crack() on {} decoders with batch size {}",
            self.components.len(),
            batch_size
        );

        let mut all_results: Vec<CrackResult> = Vec::new();
        let mut successful_results: Vec<CrackResult> = Vec::new();

        // Process decoders in batches to limit concurrency
        for chunk in self.components.chunks(batch_size) {
            // Run this batch in parallel
            let batch_results: Vec<CrackResult> = chunk
                .par_iter()
                .map(|decoder| decoder.crack(text, &checker))
                .collect();

            // Separate successful and unsuccessful results
            for result in batch_results {
                if result.success {
                    successful_results.push(result);
                } else {
                    all_results.push(result);
                }
            }
        }

        // If we have successful results, return the best one AND all other results
        if !successful_results.is_empty() {
            // Sort by popularity (highest first) - more popular decoders are more likely correct
            successful_results.sort_by(|a, b| {
                let pop_a = get_decoder_popularity_by_name(a.decoder);
                let pop_b = get_decoder_popularity_by_name(b.decoder);
                pop_b
                    .partial_cmp(&pop_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let best_result = successful_results.remove(0);
            trace!(
                "Found {} successful results, best: {} (popularity: {})",
                successful_results.len() + 1,
                best_result.decoder,
                get_decoder_popularity_by_name(best_result.decoder)
            );

            // Combine remaining successful results with unsuccessful results
            all_results.extend(successful_results);
            return MyResults::Break(best_result, all_results);
        }

        MyResults::Continue(all_results)
    }
}

/// Get decoder popularity by name for sorting results
fn get_decoder_popularity_by_name(decoder_name: &str) -> f32 {
    use crate::decoders::DECODER_MAP;

    if let Some(decoder_box) = DECODER_MAP.get(decoder_name) {
        let decoder = decoder_box.get::<()>();
        decoder.get_popularity()
    } else {
        0.5 // Default for unknown decoders
    }
}

/// [`Enum`] for our custom results.
/// `Break` contains a successful result AND all other results for continued exploration.
/// `Continue` contains only unsuccessful results.
pub enum MyResults {
    /// Variant containing successful [`CrackResult`] and all other results
    /// The first element is the best successful result, the second is all other results
    Break(CrackResult, Vec<CrackResult>),
    /// Contains [`Vec`] of [`CrackResult`] for further processing (no successes)
    Continue(Vec<CrackResult>),
}

impl MyResults {
    /// Get the successful result if any
    pub fn _break_value(self) -> Option<CrackResult> {
        match self {
            MyResults::Break(val, _) => Some(val),
            MyResults::Continue(_) => None,
        }
    }

    /// Get all results (both successful and unsuccessful) as a single vector
    pub fn all_results(self) -> Vec<CrackResult> {
        match self {
            MyResults::Break(success, mut others) => {
                others.insert(0, success);
                others
            }
            MyResults::Continue(results) => results,
        }
    }
}

/// Get all available decoders
pub fn get_all_decoders() -> Decoders {
    trace!("Getting all decoders");
    filter_and_get_decoders(&DecoderResult::default())
}

/// Currently takes no args as this is just a spike to get all the basic functionality working
pub fn filter_and_get_decoders(_text_struct: &DecoderResult) -> Decoders {
    trace!("Filtering and getting all decoders");
    let vigenere = Decoder::<VigenereDecoder>::new();
    let binary = Decoder::<BinaryDecoder>::new();
    let hexadecimal = Decoder::<HexadecimalDecoder>::new();
    let base58_bitcoin = Decoder::<Base58BitcoinDecoder>::new();
    let base58_monero = Decoder::<Base58MoneroDecoder>::new();
    let base58_ripple = Decoder::<Base58RippleDecoder>::new();
    let base58_flickr = Decoder::<Base58FlickrDecoder>::new();
    let base64 = Decoder::<Base64Decoder>::new();
    let base91 = Decoder::<Base91Decoder>::new();
    let base65536 = Decoder::<Base65536Decoder>::new();
    let citrix_ctx1 = Decoder::<CitrixCTX1Decoder>::new();
    let url = Decoder::<URLDecoder>::new();
    let base32 = Decoder::<Base32Decoder>::new();
    let base62 = Decoder::<Base62Decoder>::new();
    let reversedecoder = Decoder::<ReverseDecoder>::new();
    let morsecodedecoder = Decoder::<MorseCodeDecoder>::new();
    let atbashdecoder = Decoder::<AtbashDecoder>::new();
    let caesardecoder = Decoder::<CaesarDecoder>::new();
    let railfencedecoder = Decoder::<RailfenceDecoder>::new();
    let rot47decoder = Decoder::<ROT47Decoder>::new();
    let z85 = Decoder::<Z85Decoder>::new();
    let a1z26decoder = Decoder::<A1Z26Decoder>::new();
    let ascii85 = Decoder::<Ascii85Decoder>::new();
    let brailledecoder = Decoder::<BrailleDecoder>::new();
    let substitution_generic = Decoder::<SubstitutionGenericDecoder>::new();

    let brainfuck = Decoder::<BrainfuckInterpreter>::new();
    let nato_phonetic = Decoder::<NATOPhoneticDecoder>::new();
    let octal = Decoder::<OctalDecoder>::new();

    let mut components: Vec<Box<dyn Crack + Sync>> = vec![
        Box::new(vigenere),
        Box::new(reversedecoder),
        Box::new(base64),
        Box::new(base58_bitcoin),
        Box::new(base58_monero),
        Box::new(base58_ripple),
        Box::new(base58_flickr),
        Box::new(base91),
        Box::new(base65536),
        Box::new(binary),
        Box::new(hexadecimal),
        Box::new(base32),
        Box::new(base62),
        Box::new(morsecodedecoder),
        Box::new(atbashdecoder),
        Box::new(caesardecoder),
        Box::new(railfencedecoder),
        Box::new(citrix_ctx1),
        Box::new(url),
        Box::new(rot47decoder),
        Box::new(z85),
        Box::new(a1z26decoder),
        Box::new(ascii85),
        Box::new(brailledecoder),
        Box::new(substitution_generic),
        Box::new(brainfuck),
        Box::new(nato_phonetic),
        Box::new(octal),
    ];

    // Filter based on config.decoders_to_run if it's not empty
    let config = get_config();
    if !config.decoders_to_run.is_empty() {
        trace!("Filtering decoders to run: {:?}", config.decoders_to_run);
        components.retain(|decoder| {
            config
                .decoders_to_run
                .contains(&decoder.get_name().to_string())
        });
        trace!("After filtering: {} decoders remaining", components.len());
    }

    Decoders { components }
}

/// Get a specific decoder by name
pub fn get_decoder_by_name(decoder_name: &str) -> Decoders {
    trace!("Getting decoder by name: {}", decoder_name);
    let all_decoders = get_all_decoders();

    let filtered_components = all_decoders
        .components
        .into_iter()
        .filter(|d| d.get_name() == decoder_name)
        .collect();

    Decoders {
        components: filtered_components,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        DecoderResult,
    };

    use super::{filter_and_get_decoders, get_decoder_by_name};

    #[test]
    fn it_works() {
        let _decoders = filter_and_get_decoders(&DecoderResult::default());
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn decoders_can_call_dot_run() {
        let decoders = filter_and_get_decoders(&DecoderResult::default());
        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        decoders.run("TXIgUm9ib3QgaXMgZ3JlYXQ=", checker);
        assert_eq!(true, true);
    }

    #[test]
    fn test_get_decoder_by_name() {
        let decoder_name = "Base64";
        let decoders = get_decoder_by_name(decoder_name);

        assert_eq!(
            decoders.components.len(),
            1,
            "Should return exactly one decoder"
        );
        assert_eq!(
            decoders.components[0].get_name(),
            decoder_name,
            "Should return the requested decoder"
        );
    }

    #[test]
    fn test_get_decoder_by_name_nonexistent() {
        let decoders = get_decoder_by_name("nonexistent_decoder");
        assert!(
            decoders.components.is_empty(),
            "Should return empty decoders for nonexistent name"
        );
    }
}
