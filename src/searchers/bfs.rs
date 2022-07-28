use log::trace;
use std::collections::HashSet;
use std::time::Duration;

use crate::filtration_system::MyResults;
use crossbeam::{
    channel::{after, bounded},
    select,
};

/// Maximum duration for which we try to decoded text
const TIMEOUT_DURATION: Duration = Duration::from_secs(60);

/// Breadth first search is our search algorithm
/// https://en.wikipedia.org/wiki/Breadth-first_search
pub fn bfs(input: &str) -> Option<String> {
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![input.to_string()];

    // sender and receiver for exit result.
    // It has capacity of 1 as there will only be one successful result.
    let (result_sender, result_receiver) = bounded(1);

    // this will send after duration
    let timeout = after(TIMEOUT_DURATION);

    // loop through all of the strings in the vec
    while !current_strings.is_empty() {
        trace!("Number of potential decodings: {}", current_strings.len());

        let mut new_strings: Vec<String> = vec![];

        current_strings
            .into_iter()
            .map(|current_string| super::perform_decoding(&current_string))
            .try_for_each(|elem| match elem {
                // if it's Break variant, we have cracked the text successfully
                // so just stop processing further.
                MyResults::Break(res) => {
                    result_sender
                        .send(res)
                        .expect("expected no send error while sending exit result");
                    None // short-circuits the iterator
                }
                MyResults::Continue(results_vec) => {
                    new_strings = results_vec
                        .into_iter()
                        .flat_map(|r| r.unencrypted_text)
                        .filter(|s| seen_strings.insert(s.clone()))
                        .collect();
                    Some(()) // indicate we want to continue processing
                }
            });

        // if we find an element that matches our exit condition, return it!
        // technically this won't check if the initial string matches our exit condition
        // but this is a demo and i'll be lazy :P
        current_strings = new_strings;

        select! {
            recv(result_receiver) -> exit_result => {
                if let Ok(exit_res) = exit_result {
                    let exit_str = exit_res
                        .unencrypted_text
                        .expect("No unencrypted text even after checker succeed!");
                    trace!("Found exit string: {}", exit_str);
                    return Some(exit_str)
                }
            },
            recv(timeout) -> _ => {
                trace!("Timed out after {}s", TIMEOUT_DURATION.as_secs());
                // instead of returning None, we can change return type of bfs
                // to Result<Option<String>, TimeOutError>
                // and then we can return TimeOutError here ( it can be a String too)
                return None
            },
            default => continue,
        }

        trace!("Refreshed the vector, {:?}", current_strings);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_succeeds() {
        // this will work after english checker can identify "CANARY: hello"
        // let result = bfs("Q0FOQVJZOiBoZWxsbw==");
        // assert!(result.is_some());
        // assert!(result.unwrap() == "CANARY: hello");

        let result = bfs("b2xsZWg=");
        assert!(result.is_some());
        assert!(result.unwrap() == "hello");
    }
}
