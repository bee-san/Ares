use log::trace;
use std::collections::HashSet;

pub fn bfs(input: &str) -> Option<String> {
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![input.to_string()];

    // loop through all of the strings in the vec
    while !current_strings.is_empty() {
        trace!("Number of potential decodings: {}", current_strings.len());

        // runs the decodings and puts it into
        let all_results: Vec<_> = current_strings
            .into_iter()
            .flat_map(|current_string| super::perform_decoding(&current_string))
            .filter(|elem| seen_strings.insert(elem.unencrypted_text.clone()))
            .collect();

        // if we find an element that matches our exit condition, return it!
        // technically this won't check if the initial string matches our exit condition
        // but this is a demo and i'll be lazy :P
        if let Some(exit_res) = all_results.iter().find(|elem| elem.success) {
            let exit_str = exit_res
                .unencrypted_text
                .clone()
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
        let result = bfs("Q0FOQVJZOiBoZWxsbw==");
        assert!(result.is_some());
        assert!(result.unwrap() == "CANARY: hello")
    }
}
