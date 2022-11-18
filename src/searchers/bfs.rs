use crossbeam::{channel::bounded, select};
use log::{error, trace};
use std::collections::HashSet;

use crate::{filtration_system::MyResults, timer, Text};

/// Breadth first search is our search algorithm
/// https://en.wikipedia.org/wiki/Breadth-first_search
pub fn bfs(input: &str, max_depth: Option<u32>) -> Option<Text> {
    let initial = Text {
        text: input.to_string(),
        path: vec![],
    };
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![initial];

    // let mut exit_result: Option<Text> = None;

    let mut curr_depth: u32 = 1; // as we have input string, so we start from 1

    let timer = timer::start(1);

    let (result_send, result_recv) = bounded(1);

    // loop through all of the strings in the vec
    while !current_strings.is_empty() && max_depth.map_or(true, |x| curr_depth <= x) {
        trace!("Number of potential decodings: {}", current_strings.len());
        trace!("Current depth is {:?}; [ {:?} max ]", curr_depth, max_depth);

        let mut new_strings: Vec<Text> = vec![];

        current_strings.into_iter().try_for_each(|current_string| {
            let res = super::perform_decoding(&current_string.text);

            match res {
                // if it's Break variant, we have cracked the text successfully
                // so just stop processing further.
                MyResults::Break(res) => {
                    let mut decoders_used = current_string.path;
                    decoders_used.push(res.decoder);
                    let result_text = Text {
                        text: res.unencrypted_text.unwrap_or_default(),
                        path: decoders_used,
                    };

                    result_send
                        .send(result_text)
                        .expect("Succesfully send the result");
                    // exit_result = Some(result_text);
                    None // short-circuits the iterator
                }
                MyResults::Continue(results_vec) => {
                    new_strings.extend(
                        results_vec
                            .into_iter()
                            .map(|r| {
                                let mut decoders_used = current_string.path.clone();
                                decoders_used.push(r.decoder);
                                Text {
                                    text: r.unencrypted_text.unwrap_or_default(),
                                    path: decoders_used,
                                }
                            })
                            .filter(|s| seen_strings.insert(s.text.clone())),
                    );
                    Some(()) // indicate we want to continue processing
                }
            }
        });
        current_strings = new_strings;
        curr_depth += 1;

        select! {
            recv(result_recv) -> exit_result => {
                // if we find an element that matches our exit condition, return it!
                // technically this won't check if the initial string matches our exit condition
                // but this is a demo and i'll be lazy :P
                let exit_result = exit_result.ok(); // convert Result to Some
                if exit_result.is_some() {
                    trace!("Found exit result: {:?}", exit_result);
                    return exit_result;
                }
            },
            recv(timer) -> _ => {
                error!("TIMEOUT!!!");
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
        // let result = bfs("Q0FOQVJZOiBoZWxsbw==");
        // assert!(result.is_some());
        // assert!(result.unwrap() == "CANARY: hello");
        let result = bfs("b2xsZWg=", None);
        assert!(result.is_some());
        assert!(result.unwrap().text == "hello");
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
        let result = bfs(
            "UFRCRVRWVkNiRlZMTVVkYVVFWjZVbFZPU0\
        dGMU1WVlpZV2d4VkRVNWJWWnJjRzFVUlhCc1pYSlNWbHBPY0VaV1ZXeHJWRWd4TUZWdlZsWlg=",
            None,
        );
        assert!(result.is_some());
        assert_eq!(result.unwrap().text, "https://www.google.com");
    }

    #[test]
    fn max_depth_test() {
        // text is encoded with base64 5 times
        let result = bfs("VjFaV2ExWXlUWGxUYTJoUVVrUkJPUT09", Some(4));
        // It goes only upto depth 4, so it can't find the plaintext
        assert!(result.is_none());
    }
}
