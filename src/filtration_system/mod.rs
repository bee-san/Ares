//! Proposal: https://broadleaf-angora-7db.notion.site/Filtration-System-7143b36a42f1466faea3077bfc7e859e
//! Given a filter object, return an array of decoders/crackers which have been filtered

use crate::checkers::CheckerTypes;
use crate::decoders::crack_results::CrackResult;
use crate::decoders::DECODER_MAP;
use crate::DecoderResult;

use crate::config::get_config;
use log::trace;
use rayon::prelude::*;

/// The struct which contains all of the decoders.
///
/// Instead of holding boxed trait objects, this now stores decoder names
/// and looks them up in `DECODER_MAP` at runtime. This removes the need to
/// manually instantiate every decoder in a giant list  adding a decoder to
/// `DECODER_MAP` automatically makes it available here.
pub struct Decoders {
    /// Decoder names referencing entries in `DECODER_MAP`.
    pub components: Vec<&'static str>,
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
            // Run this batch in parallel, catching panics from individual decoders
            // (e.g., ascii85 crate can panic with "attempt to add with overflow")
            let batch_results: Vec<CrackResult> = chunk
                .par_iter()
                .filter_map(|decoder_name| {
                    let decoder_box = DECODER_MAP.get(decoder_name)?;
                    let decoder = decoder_box.get::<()>();
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        decoder.crack(text, &checker)
                    }))
                    .ok()
                })
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

/// Build the decoder list dynamically from `DECODER_MAP`, applying config
/// filters. This replaces the old approach of manually instantiating every
/// decoder, which had to be updated every time a new decoder was added.
pub fn filter_and_get_decoders(_text_struct: &DecoderResult) -> Decoders {
    trace!("Filtering and getting all decoders from DECODER_MAP");

    let config = get_config();

    // Collect decoder names, excluding the internal "Default decoder"
    let mut decoder_names: Vec<&str> = DECODER_MAP
        .keys()
        .copied()
        .filter(|name| *name != "Default decoder")
        .collect();

    // Sort for deterministic ordering
    decoder_names.sort();

    // Filter based on config.decoders_to_run if it's not empty
    if !config.decoders_to_run.is_empty() {
        trace!("Filtering decoders to run: {:?}", config.decoders_to_run);
        decoder_names.retain(|name| config.decoders_to_run.contains(&name.to_string()));
        trace!(
            "After filtering: {} decoders remaining",
            decoder_names.len()
        );
    }

    trace!(
        "Building decoder list with {} decoders: {:?}",
        decoder_names.len(),
        decoder_names
    );

    Decoders {
        components: decoder_names,
    }
}

/// Get a specific decoder by name
pub fn get_decoder_by_name(decoder_name: &str) -> Decoders {
    trace!("Getting decoder by name: {}", decoder_name);

    // Check if the decoder exists in DECODER_MAP
    if DECODER_MAP.contains_key(decoder_name) {
        Decoders {
            components: vec![DECODER_MAP
                .keys()
                .find(|&&k| k == decoder_name)
                .copied()
                .unwrap()],
        }
    } else {
        Decoders {
            components: vec![],
        }
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
            decoders.components[0], decoder_name,
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