use crate::filtration_system::MyResults;
use crate::{cli_pretty_printing::decoded_how_many_times, config::get_config};
use crossbeam::{channel::bounded, select};
use log::{debug, trace};
use std::collections::HashSet;

use crate::{timer, DecoderResult};

/// Breadth first search is our search algorithm
/// https://en.wikipedia.org/wiki/Breadth-first_search
pub fn bfs(input: &str) -> Option<DecoderResult> {
    let config = get_config();
    let initial = DecoderResult {
        text: vec![input.to_string()],
        path: vec![],
    };
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![initial];

    let mut curr_depth: u32 = 1; // as we have input string, so we start from 1

    let (result_send, result_recv) = bounded(1);
    let timer = timer::start(config.timeout);

    // loop through all of the strings in the vec
    while !current_strings.is_empty() {
        trace!("Number of potential decodings: {}", current_strings.len());
        trace!("Current depth is {:?}", curr_depth);

        let mut new_strings: Vec<DecoderResult> = vec![];

        current_strings.into_iter().try_for_each(|current_string| {
            let res = super::perform_decoding(&current_string);

            match res {
                // if it's Break variant, we have cracked the text successfully
                // so just stop processing further.
                MyResults::Break(res) => {
                    let mut decoders_used = current_string.path;
                    let text = res.unencrypted_text.clone().unwrap_or_default();
                    decoders_used.push(res);
                    let result_text = DecoderResult {
                        text,
                        path: decoders_used,
                    };

                    result_send
                        .send(result_text)
                        .expect("Succesfully send the result");
                    None // short-circuits the iterator
                }
                MyResults::Continue(results_vec) => {
                    new_strings.extend(
                        results_vec
                            .into_iter()
                            .map(|r| {
                                let mut decoders_used = current_string.path.clone();
                                // text is a vector of strings
                                let text = r.unencrypted_text.clone().unwrap_or_default();
                                decoders_used.push(r);
                                DecoderResult {
                                    // and this is a vector of strings
                                    // TODO we should probably loop through all `text` and create Text structs for each one
                                    // and append those structs
                                    // I think we should keep text as a single string
                                    // and just create more of them....
                                    text,
                                    path: decoders_used.to_vec(),
                                }
                            })
                            .filter(|s| seen_strings.insert(s.text.clone())),
                    );
                    Some(()) // indicate we want to continue processing
                }
            }
        });
        let mut new_strings_to_be_added = Vec::new();
        for text_struct in new_strings {
            for decoded_text in text_struct.text {
                if check_if_string_cant_be_decoded(&decoded_text) {
                    continue;
                }
                new_strings_to_be_added.push(DecoderResult {
                    text: vec![decoded_text],
                    // quick hack
                    path: text_struct.path.clone(),
                })
            }
        }
        current_strings = new_strings_to_be_added;
        curr_depth += 1;

        select! {
            recv(result_recv) -> exit_result => {
                // if we find an element that matches our exit condition, return it!
                // technically this won't check if the initial string matches our exit condition
                // but this is a demo and i'll be lazy :P
                let exit_result = exit_result.ok(); // convert Result to Some
                if exit_result.is_some() {
                    decoded_how_many_times(curr_depth);
                    debug!("Found exit result: {:?}", exit_result);
                    return exit_result;
                }
            },
            recv(timer) -> _ => {
                decoded_how_many_times(curr_depth);
                debug!("Ares has failed to decode");
                return None;
            },
            default => continue,
        };

        trace!("Refreshed the vector, {:?}", current_strings);
    }

    None
}

/// If this returns False it will not attempt to decode that string
fn check_if_string_cant_be_decoded(text: &str) -> bool {
    text.len() <= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_succeeds() {
        // this will work after english checker can identify "CANARY: hello"
        let result = bfs("b2xsZWg=");
        assert!(result.is_some());
        let txt = result.unwrap().text;
        assert!(txt[0] == "hello");
    }

    // Vector storing the strings to perform decoding in next iteraion
    // had strings only from result of last decoding it performed.
    // This was due to reassignment in try_for_each block
    // which lead to unintended behaviour.
    // We want strings from all results, so to fix it,
    // we call .extend() to extend the vector.
    // Link to forum https://discord.com/channels/754001738184392704/1002135076034859068
    // This also tests the bug whereby each iteration of caesar was not passed to the next decoder
    // So in Ciphey only Rot1(X) was passed to base64, not Rot13(X)
    #[test]
    fn non_deterministic_like_behaviour_regression_test() {
        // Caesar Cipher (Rot13) -> Base64
        let result = bfs("MTkyLjE2OC4wLjE=");
        assert!(result.is_some());
        assert_eq!(result.unwrap().text[0], "192.168.0.1");
    }

    #[test]
    fn string_size_checker_returns_bad_if_string_cant_be_decoded() {
        // Should return true because it cant decode it
        let text = "12";
        assert!(check_if_string_cant_be_decoded(text));
    }

    #[test]
    fn string_size_checker_returns_ok_if_string_can_be_decoded() {
        // Should return true because it cant decode it
        let text = "123";
        assert!(!check_if_string_cant_be_decoded(text));
    }
}
