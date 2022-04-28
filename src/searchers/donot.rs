use base64; // 0.13.0
use percent_encoding;
use std::collections::HashSet; // 2.1.0;

fn main() {
    if let Some(output) =
        bfs("=UFVCdEVxY0VTxGcQFWVKp0VsRGNjJjS1I1VkdlUyg2dZNDbDN2ROVTUthmSSFjSzlFbjRjWwkjcVRFMkNTJ")
    {
        println!("Result:\n\t\"{}\"", output);
    } else {
        println!("No results...");
    }
}

fn bfs(input: &str) -> Option<String> {
    let mut seen_strings = HashSet::new();
    // all strings to search through
    let mut current_strings = vec![input.to_string()];

    // loop through all of the strings in the vec
    while !current_strings.is_empty() {
        println!("Number of potential decodings: {}", current_strings.len());

        // runs the decodings and puts it into
        let new_strings: Vec<_> = current_strings
            .into_iter()
            .flat_map(|current_string| perform_decoding(&current_string))
            .filter_map(|elem| elem)
            .filter(|elem| seen_strings.insert(elem.to_string()))
            .collect();

        // if we find an element that matches our exit condition, return it!
        // technically this won't check if the initial string matches our exit condition
        // but this is a demo and i'll be lazy :P
        if let Some(exit_str) = new_strings.iter().find(|elem| exit_condition(elem)) {
            return Some(exit_str.to_string());
        }

        current_strings = new_strings;
    }

    None
}

// https://github.com/bee-san/Ares/pull/14/files#diff-b8829c7e292562666c7fa5934de7b478c4a5de46d92e42c46215ac4d9ff89db2R37
fn exit_condition(input: &str) -> bool {
    // use your exit condition. I'll put in a fake one here.

    // checks that the string starts with a canary message
    input.starts_with("CANARY:")
}

fn perform_decoding(input: &str) -> Vec<Option<String>> {
    // sub in your implementation here
    // I'm using a static vec like this for demo purposes
    vec![
        B64::decode(input),
        URL::decode(input),
        Reverse::decode(input),
    ]
}

// =============================================================================
/* just some stuff for the demo */
// =============================================================================
struct B64 {}

struct URL {}

struct Reverse {}

trait Decoder {
    fn decode(input: &str) -> Option<String>;
}

impl Decoder for B64 {
    fn decode(input: &str) -> Option<String> {
        base64::decode(input.as_bytes())
            .ok()
            .map(|inner| String::from_utf8(inner).ok())?
    }
}

impl Decoder for URL {
    fn decode(input: &str) -> Option<String> {
        let output = percent_encoding::percent_decode_str(input)
            .decode_utf8()
            .ok()
            .map(|inner| inner.to_string());

        // if url decoding doesn't change the string, we'll assume it did "nothing"
        match output {
            Some(inner_output) if inner_output != input => Some(inner_output),
            _ => None,
        }
    }
}

impl Decoder for Reverse {
    fn decode(input: &str) -> Option<String> {
        Some(input.chars().rev().collect())
    }
}
