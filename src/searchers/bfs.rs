use log::trace;
use std::collections::HashSet;

use crate::decoders::crack_results::CrackResult;

pub fn bfs(input: &str) -> Option<String> {
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![input.to_string()];

    // loop through all of the strings in the vec
    while !current_strings.is_empty() {
        trace!("Number of potential decodings: {}", current_strings.len());

        let mut exit_result: Option<CrackResult> = None;

        // have capacity to avoid reallocatoins
        let mut all_results = Vec::with_capacity(current_strings.len());

        current_strings
            .into_iter()
            .flat_map(|current_string| super::perform_decoding(&current_string))
            .try_for_each(|elem| {
                if elem.success {
                    exit_result = Some(elem);
                    // short-circuit if we met exit condition
                    return None;
                }

                if seen_strings.insert(elem.unencrypted_text.clone()) {
                    all_results.push(elem);
                }
                Some(())
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

        current_strings = all_results
            .iter()
            .filter_map(|res| res.unencrypted_text.clone())
            .collect();
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
