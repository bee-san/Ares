use log::trace;
use std::collections::HashSet;

use crate::{decoders::crack_results::CrackResult, filtration_system::MyResults};

/// Breadth first search is our search algorithm
/// https://en.wikipedia.org/wiki/Breadth-first_search
pub fn bfs(input: &str) -> Option<String> {
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![input.to_string()];

    let mut exit_result: Option<CrackResult> = None;

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
                    exit_result = Some(res);
                    None // short-circuits the iterator
                }
                MyResults::Continue(results_vec) => {
                    new_strings.extend(
                        results_vec
                            .into_iter()
                            .flat_map(|r| r.unencrypted_text)
                            .filter(|s| seen_strings.insert(s.clone())),
                    );
                    Some(()) // indicate we want to continue processing
                }
            });

        // if we find an element that matches our exit condition, return it!
        // technically this won't check if the initial string matches our exit condition
        // but this is a demo and i'll be lazy :P
        if let Some(exit_res) = exit_result {
            let exit_str = exit_res
                .unencrypted_text
                .expect("No unencrypted text even after checker succeed!");
            trace!("Found exit string: {}", exit_str);
            return Some(exit_str);
        }

        current_strings = new_strings;

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

    // Vector storing the strings to perform decoding in next iteraion
    // had strings only from result of last decoding it performed.
    // This was due to reassignment in try_for_each block
    // which lead to unintended behaviour.
    // We want strings from all results, so to fix it,
    // we call .extend() to extend the vector.
    // Link to forum https://discord.com/channels/754001738184392704/1002135076034859068
    #[test]
    fn non_deterministic_like_behaviour_regression_test() {
        // text was too long, so we put \ to escape the \n
        // and put the rest of string on next line.
        let result = bfs("UFRCRVRWVkNiRlZMTVVkYVVFWjZVbFZPU0\
        dGMU1WVlpZV2d4VkRVNWJWWnJjRzFVUlhCc1pYSlNWbHBPY0VaV1ZXeHJWRWd4TUZWdlZsWlg=");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "https://www.google.com");
    }
}
