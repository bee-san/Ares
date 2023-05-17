//! The search algorithm decides what encryptions to do next
//! And also runs the decryption modules
//! Click here to find out more:
//! https://broadleaf-angora-7db.notion.site/Search-Nodes-Edges-What-should-they-look-like-b74c43ca7ac341a1a5cfdbeb84a7eef0

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;

use crossbeam::channel::bounded;
use log::debug;

use crate::checkers::athena::Athena;
use crate::checkers::checker_type::{Check, Checker};
use crate::checkers::CheckerTypes;
use crate::config::get_config;
use crate::filtration_system::{filter_and_get_decoders, CrackResults};
use crate::{timer, DecoderResult};
/// This module provides access to the breadth first search
/// which searches for the plaintext.
mod bfs;

/*pub struct Tree <'a> {
    // Wrap in a box because
    // https://doc.rust-lang.org/error-index.html#E0072
    parent: &'a Box<Option<Tree<'a>>>,
    value: String
}*/

/// Performs the search algorithm.
///
/// When we perform the decryptions, we will get a vector of Some<String>
/// We need to loop through these and determine:
/// 1. Did we reach our exit condition?
/// 2. If not, create new nodes out of them and add them to the queue.
/// We can return an Option? An Enum? And then match on that
/// So if we return CrackSuccess we return
/// Else if we return an array, we add it to the children and go again.
pub fn search_for_plaintext(input: String) -> Option<DecoderResult> {
    let timeout = get_config().timeout;
    let timer = timer::start(timeout);

    let (result_sender, result_recv) = bounded::<Option<DecoderResult>>(1);
    // For stopping the thread
    let stop = Arc::new(AtomicBool::new(false));
    let s = stop.clone();
    // Change this to select which search algorithm we want to use.
    let handle = thread::spawn(move || bfs::bfs(input, result_sender, s));

    loop {
        if let Ok(res) = result_recv.try_recv() {
            debug!("Found exit result: {:?}", res);
            handle.join().unwrap();
            return res;
        }

        if timer.try_recv().is_ok() {
            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            debug!("Ares has failed to decode");
            // this would wait for whole iteration to finish!
            // handle.join().unwrap();
            return None;
        }
    }
}

/// Performs the decodings by getting all of the decoders
/// and calling `.run` which in turn loops through them and calls
/// `.crack()`.
fn perform_decoding(text: &DecoderResult) -> CrackResults {
    let decoders = filter_and_get_decoders(text);
    let athena_checker = Checker::<Athena>::new();
    let checker = CheckerTypes::CheckAthena(athena_checker);
    decoders.run(&text.text[0], checker)
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://github.com/bee-san/Ares/pull/14/files#diff-b8829c7e292562666c7fa5934de7b478c4a5de46d92e42c46215ac4d9ff89db2R37
    // Only used for tests!
    fn exit_condition(input: &str) -> bool {
        // use Athena Checker from checkers module
        // call check(input)
        let athena_checker = Checker::<Athena>::new();
        let checker = CheckerTypes::CheckAthena(athena_checker);
        checker.check(input).is_identified
    }

    #[test]
    fn exit_condition_succeeds() {
        let result = exit_condition("https://www.google.com");
        assert!(result);
    }
    #[test]
    fn exit_condition_fails() {
        let result = exit_condition("vjkrerkdnxhrfjekfdjexk");
        assert!(!result);
    }

    #[test]
    fn perform_decoding_succeeds() {
        let dc = DecoderResult::_new("aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbQ==");
        let result = perform_decoding(&dc);
        assert!(
            result
                .break_value()
                .expect("expected successful value, none found")
                .success
        );
        //TODO assert that the plaintext is correct by looping over the vector
    }
    #[test]
    fn perform_decoding_succeeds_empty_string() {
        // Some decoders like base64 return even when the string is empty.
        let dc = DecoderResult::_new("");
        let result = perform_decoding(&dc);
        assert!(result.break_value().is_none());
    }
}
