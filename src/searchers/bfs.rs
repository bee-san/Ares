use crate::config::get_config;
use crossbeam::{channel::bounded, select};
use log::{error, trace};
use std::collections::HashSet;

use crate::{filtration_system::MyResults, timer, Text};

/// Breadth first search is our search algorithm
/// https://en.wikipedia.org/wiki/Breadth-first_search
pub fn bfs(input: &str) -> Option<Text> {
    let config = get_config();
    let initial = Text {
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

        let mut new_strings: Vec<Text> = vec![];

        current_strings.into_iter().try_for_each(|current_string| {
            let res = super::perform_decoding(&current_string.text[0]);

            match res {
                // if it's Break variant, we have cracked the text successfully
                // so just stop processing further.
                MyResults::Break(res) => {
                    let mut decoders_used = current_string.path;
                    let text = res.unencrypted_text.clone().unwrap_or_default();
                    decoders_used.push(res);
                    let result_text = Text {
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
                                Text {
                                    // and this is a vector of strings
                                    // TODO we should probably loop through all `text` and create Text structs for each one
                                    // and append those structs
                                    // I think we should keep text as a single string
                                    // and just create more of them....
                                    text,
                                    path: decoders_used,
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
                new_strings_to_be_added.push(Text {
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
                    println!("Ares has decoded {times} times", times=curr_depth * (12+25));
                    trace!("Found exit result: {:?}", exit_result);
                    return exit_result;
                }
            },
            recv(timer) -> _ => {
                println!("Ares has decoded {times} times", times=curr_depth * (12+25));
                error!("Ares failed to decrypt the text you have provided within {} seconds, it is unlikely to be decoded.", config.timeout);
                return None;
            },
            default => continue,
        };

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
}
