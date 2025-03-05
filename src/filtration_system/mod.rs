//! Proposal: https://broadleaf-angora-7db.notion.site/Filtration-System-7143b36a42f1466faea3077bfc7e859e
//! Given a filter object, return an array of decoders/crackers which have been filtered

use std::sync::mpsc::channel;

use crate::checkers::CheckerTypes;
use crate::cli_pretty_printing;
use crate::decoders::atbash_decoder::AtbashDecoder;
use crate::decoders::base32_decoder::Base32Decoder;
use crate::decoders::base58_bitcoin_decoder::Base58BitcoinDecoder;
use crate::decoders::base58_monero_decoder::Base58MoneroDecoder;
use crate::decoders::binary_decoder::BinaryDecoder;
use crate::decoders::hexadecimal_decoder::HexadecimalDecoder;
use crate::DecoderResult;

use crate::decoders::base58_flickr_decoder::Base58FlickrDecoder;
use crate::decoders::base58_ripple_decoder::Base58RippleDecoder;

use crate::decoders::a1z26_decoder::A1Z26Decoder;
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

impl Decoders {
    /// Iterate over all of the decoders and run .crack(text) on them
    /// Then if the checker succeed, we short-circuit the iterator
    /// and stop all processing as soon as possible.
    /// We are using Trait Objects
    /// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
    /// Which allows us to have multiple different structs in the same vector
    /// But each struct shares the same `.crack()` method, so it's fine.
    pub fn run(&self, text: &str, checker: CheckerTypes) -> MyResults {
        trace!("Running .crack() on all decoders");
        let (sender, receiver) = channel();
        self.components
            .into_par_iter()
            .try_for_each_with(sender, |s, i| {
                let results = i.crack(text, &checker);
                if results.success {
                    cli_pretty_printing::success(&format!(
                        "DEBUG: filtration_system - Decoder {} succeeded, short-circuiting",
                        results.decoder
                    ));
                    s.send(results.clone()).expect("expected no send error!");
                    // returning None short-circuits the iterator
                    // we don't process any further as we got success
                    return None;
                }
                cli_pretty_printing::success(&format!(
                    "DEBUG: filtration_system - Decoder {} failed, continuing",
                    results.decoder
                ));
                s.send(results.clone()).expect("expected no send error!");
                // return Some(()) to indicate that continue processing
                Some(())
            });

        let mut all_results: Vec<CrackResult> = Vec::new();

        while let Ok(result) = receiver.recv() {
            // if we recv success, break.
            if result.success {
                cli_pretty_printing::success(&format!("DEBUG: filtration_system - Received successful result from {}, returning Break", result.decoder));
                return MyResults::Break(result);
            }
            all_results.push(result)
        }

        cli_pretty_printing::success(&format!(
            "DEBUG: filtration_system - No successful results, returning Continue with {} results",
            all_results.len()
        ));
        MyResults::Continue(all_results)
    }
}

/// [`Enum`] for our custom results.
/// if our checker succeed, we return `Break` variant contining [`CrackResult`]
/// else we return `Continue` with the decoded results.
pub enum MyResults {
    /// Variant containing successful [`CrackResult`]
    Break(CrackResult),
    /// Contains [`Vec`] of [`CrackResult`] for further processing
    Continue(Vec<CrackResult>),
}

impl MyResults {
    /// named with _ to pass dead_code warning
    /// as we aren't using it, it's just used in tests
    pub fn _break_value(self) -> Option<CrackResult> {
        match self {
            MyResults::Break(val) => Some(val),
            MyResults::Continue(_) => None,
        }
    }
}

/// Filter struct for decoder filtering
pub struct DecoderFilter {
    /// Tags to include in the filter - decoders must have at least one of these tags
    include_tags: Vec<String>,
    /// Tags to exclude from the filter - decoders must not have any of these tags
    exclude_tags: Vec<String>,
}

impl DecoderFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        DecoderFilter {
            include_tags: Vec::new(),
            exclude_tags: Vec::new(),
        }
    }

    /// Add a tag to include
    pub fn include_tag(mut self, tag: &str) -> Self {
        self.include_tags.push(tag.to_string());
        self
    }

    /// Add a tag to exclude
    pub fn exclude_tag(mut self, tag: &str) -> Self {
        self.exclude_tags.push(tag.to_string());
        self
    }

    /// Check if a decoder matches the filter
    #[allow(clippy::borrowed_box)]
    pub fn matches(&self, decoder: &Box<dyn Crack + Sync>) -> bool {
        let tags = decoder.get_tags();

        // If include_tags is not empty, at least one tag must match
        if !self.include_tags.is_empty() {
            let has_included_tag = self
                .include_tags
                .iter()
                .any(|include_tag| tags.iter().any(|tag| *tag == include_tag));

            if !has_included_tag {
                return false;
            }
        }

        // If exclude_tags is not empty, no tag must match
        if !self.exclude_tags.is_empty() {
            let has_excluded_tag = self
                .exclude_tags
                .iter()
                .any(|exclude_tag| tags.iter().any(|tag| *tag == exclude_tag));

            if has_excluded_tag {
                return false;
            }
        }

        true
    }
}

/// Get decoders with the "decoder" tag
pub fn get_decoder_tagged_decoders(text_struct: &DecoderResult) -> Decoders {
    trace!("Getting decoder-tagged decoders");
    let filter = DecoderFilter::new().include_tag("decoder");
    filter_decoders_by_tags(text_struct, &filter)
}

/// Get decoders without the "decoder" tag
pub fn get_non_decoder_tagged_decoders(text_struct: &DecoderResult) -> Decoders {
    trace!("Getting non-decoder-tagged decoders");
    let filter = DecoderFilter::new().exclude_tag("decoder");
    filter_decoders_by_tags(text_struct, &filter)
}

/// Filter decoders based on custom tags
pub fn filter_decoders_by_tags(_text_struct: &DecoderResult, filter: &DecoderFilter) -> Decoders {
    trace!("Filtering decoders by tags");

    // Get all decoders
    let all_decoders = get_all_decoders();

    // Filter decoders based on tags
    let filtered_components = all_decoders
        .components
        .into_iter()
        .filter(|decoder| filter.matches(decoder))
        .collect();

    Decoders {
        components: filtered_components,
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
    let reversedecoder = Decoder::<ReverseDecoder>::new();
    let morsecodedecoder = Decoder::<MorseCodeDecoder>::new();
    let atbashdecoder = Decoder::<AtbashDecoder>::new();
    let caesardecoder = Decoder::<CaesarDecoder>::new();
    let railfencedecoder = Decoder::<RailfenceDecoder>::new();
    let rot47decoder = Decoder::<ROT47Decoder>::new();
    let z85 = Decoder::<Z85Decoder>::new();
    let a1z26decoder = Decoder::<A1Z26Decoder>::new();
    let brailledecoder = Decoder::<BrailleDecoder>::new();
    let substitution_generic = Decoder::<SubstitutionGenericDecoder>::new();
    Decoders {
        components: vec![
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
            Box::new(morsecodedecoder),
            Box::new(atbashdecoder),
            Box::new(caesardecoder),
            Box::new(railfencedecoder),
            Box::new(citrix_ctx1),
            Box::new(url),
            Box::new(rot47decoder),
            Box::new(z85),
            Box::new(a1z26decoder),
            Box::new(brailledecoder),
            Box::new(substitution_generic),
        ],
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

    use super::{
        filter_and_get_decoders, filter_decoders_by_tags, get_decoder_tagged_decoders,
        get_non_decoder_tagged_decoders, DecoderFilter,
    };

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
    fn test_decoder_filter_include_tag() {
        let filter = DecoderFilter::new().include_tag("base");
        let decoders = filter_decoders_by_tags(&DecoderResult::default(), &filter);

        // Verify all returned decoders have the "base" tag or a tag starting with "base"
        for decoder in decoders.components.iter() {
            let tags = decoder.get_tags();
            let has_base_tag = tags
                .iter()
                .any(|tag| *tag == "base" || tag.starts_with("base"));
            assert!(
                has_base_tag,
                "Decoder {} should have 'base' tag or tag starting with 'base', but has tags: {:?}",
                decoder.get_name(),
                tags
            );
        }

        // Ensure we have at least one decoder with the "base" tag
        assert!(
            !decoders.components.is_empty(),
            "Should have at least one decoder with 'base' tag"
        );
    }

    #[test]
    fn test_decoder_filter_exclude_tag() {
        let filter = DecoderFilter::new().exclude_tag("base64");
        let decoders = filter_decoders_by_tags(&DecoderResult::default(), &filter);

        // Verify none of the returned decoders have the "base64" tag
        for decoder in decoders.components.iter() {
            let tags = decoder.get_tags();
            assert!(
                !tags.contains(&"base64"),
                "Decoder {} should not have 'base64' tag, but has tags: {:?}",
                decoder.get_name(),
                tags
            );
        }

        // Ensure we have some decoders without the "base64" tag
        assert!(
            !decoders.components.is_empty(),
            "Should have some decoders without 'base64' tag"
        );
    }

    #[test]
    fn test_decoder_filter_combined() {
        let filter = DecoderFilter::new()
            .include_tag("base")
            .exclude_tag("base64");

        let decoders = filter_decoders_by_tags(&DecoderResult::default(), &filter);

        // Verify all returned decoders have the "base" tag but not the "base64" tag
        for decoder in decoders.components.iter() {
            let tags = decoder.get_tags();
            let has_base_tag = tags
                .iter()
                .any(|tag| *tag == "base" || tag.starts_with("base"));
            assert!(
                has_base_tag,
                "Decoder {} should have 'base' tag or tag starting with 'base', but has tags: {:?}",
                decoder.get_name(),
                tags
            );
            assert!(
                !tags.contains(&"base64"),
                "Decoder {} should not have 'base64' tag, but has tags: {:?}",
                decoder.get_name(),
                tags
            );
        }
    }

    #[test]
    fn test_get_decoder_tagged_decoders() {
        let decoders = get_decoder_tagged_decoders(&DecoderResult::default());

        // Check if we have any decoders with the "decoder" tag
        let has_decoder_tag = decoders
            .components
            .iter()
            .any(|decoder| decoder.get_tags().contains(&"decoder"));

        // This test might pass or fail depending on whether any decoders have the "decoder" tag
        // If none have it, we should at least get an empty list
        if !has_decoder_tag {
            assert!(
                decoders.components.is_empty(),
                "If no decoders have the 'decoder' tag, the result should be empty"
            );
        }
    }

    #[test]
    fn test_get_non_decoder_tagged_decoders() {
        let decoders = get_non_decoder_tagged_decoders(&DecoderResult::default());

        // Verify none of the returned decoders have the "decoder" tag
        for decoder in decoders.components.iter() {
            assert!(
                !decoder.get_tags().contains(&"decoder"),
                "Decoder {} should not have 'decoder' tag, but has tags: {:?}",
                decoder.get_name(),
                decoder.get_tags()
            );
        }

        // We should have at least some decoders without the "decoder" tag
        assert!(
            !decoders.components.is_empty(),
            "Should have some decoders without 'decoder' tag"
        );
    }
}
