use std::sync::mpsc::channel;

use crate::checkers::CheckerTypes;
use crate::DecoderResult;

use crate::decoders::crack_results::CrackResult;
use crate::decoders::interface::{Crack, Decoder};
///! Proposal: https://broadleaf-angora-7db.notion.site/Filtration-System-7143b36a42f1466faea3077bfc7e859e
///! Given a filter object, return an array of decoders/crackers which have been filtered
///

/// Import all of the decoders
use crate::decoders::{
    atbash_decoder::AtbashDecoder, base32_decoder::Base32Decoder,
    base58_bitcoin_decoder::Base58BitcoinDecoder, base58_flickr_decoder::Base58FlickrDecoder,
    base58_monero_decoder::Base58MoneroDecoder, base58_ripple_decoder::Base58RippleDecoder,
    base64_decoder::Base64Decoder, base64_url_decoder::Base64URLDecoder,
    base65536_decoder::Base65536Decoder, base91_decoder::Base91Decoder,
    binary_decoder::BinaryDecoder, caesar_decoder::CaesarDecoder,
    citrix_ctx1_decoder::CitrixCTX1Decoder, hexadecimal_decoder::HexadecimalDecoder,
    morse_code::MorseCodeDecoder, railfence_decoder::RailfenceDecoder,
    reverse_decoder::ReverseDecoder, url_decoder::URLDecoder,
};

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
    pub fn run(&self, text: &str, checker: CheckerTypes) -> CrackResults {
        trace!("Running .crack() on all decoders");
        let (sender, receiver) = channel();
        self.components
            .into_par_iter()
            .try_for_each_with(sender, |s, i| -> Option<()> {
                let results = i.crack(text, &checker);
                if results.success {
                    s.send(results).expect("Failed to send results!");
                    return None; // Short-circuit the iterator
                }
                s.send(results).expect("Failed to send results!");
                Some(()) // Continue the iterator
            });

        let mut all_results: Vec<CrackResult> = Vec::new();

        for result in receiver.iter() {
            if result.success {
                return CrackResults::Break(result);
            }
            all_results.push(result);
        }

        CrackResults::Continue(all_results)
    }
}

/// Enum representing the result of a cracking operation.
/// If the checker succeeds, it returns the `Break` variant containing `CrackResult`.
/// Otherwise, it returns the `Continue` variant with a vector of `CrackResult` for further processing.
pub enum CrackResults {
    /// Variant containing a successful `CrackResult`.
    Break(CrackResult),
    /// Contains a vector of `CrackResult` for further processing.
    Continue(Vec<CrackResult>),
}

impl CrackResults {
    #[allow(dead_code)]
    /// Returns the `CrackResult` if the checker succeeds.
    pub fn break_value(self) -> Option<CrackResult> {
        match self {
            CrackResults::Break(val) => Some(val),
            CrackResults::Continue(_) => None,
        }
    }
}

/// Create a decoder and return it as a Box<dyn Crack>
macro_rules! create_decoder {
    ($decoder:ty) => {
        Box::new(Decoder::<$decoder>::new())
    };
}

/// Currently takes no args as this is just a spike to get all the basic functionality working
pub fn filter_and_get_decoders(_text_struct: &DecoderResult) -> Decoders {
    trace!("Filtering and getting all decoders");
    Decoders {
        components: vec![
            create_decoder!(ReverseDecoder),
            create_decoder!(Base64Decoder),
            create_decoder!(Base58BitcoinDecoder),
            create_decoder!(Base58MoneroDecoder),
            create_decoder!(Base58RippleDecoder),
            create_decoder!(Base58FlickrDecoder),
            create_decoder!(Base91Decoder),
            create_decoder!(Base65536Decoder),
            create_decoder!(BinaryDecoder),
            create_decoder!(HexadecimalDecoder),
            create_decoder!(Base32Decoder),
            create_decoder!(MorseCodeDecoder),
            create_decoder!(AtbashDecoder),
            create_decoder!(CaesarDecoder),
            create_decoder!(RailfenceDecoder),
            create_decoder!(CitrixCTX1Decoder),
            create_decoder!(URLDecoder),
            create_decoder!(Base64URLDecoder),
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

    // TODO: when we add a proper filtration system
    // We need to test that.
    use super::filter_and_get_decoders;

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
}
