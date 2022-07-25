use log::{debug, trace};

use std::collections::HashMap;

use super::interface::Crack;
use super::interface::Decoder;

use serde::Deserialize;

pub struct HashDecoder;

impl Crack for Decoder<HashDecoder> {
    fn new() -> Decoder<HashDecoder> {
        Decoder {
            name: "hash",
            description: "A cryptographic hash function is a hash function which takes an input (or 'message') and returns a fixed-size string of bytes. The string is called the 'hash value', 'message digest', 'digital fingerprint', 'digest' or 'checksum'.",
            link: "https://en.wikipedia.org/wiki/Hash_function",
            tags: vec!["hash", "decoder"],
            expected_runtime: 0.8,
            expected_success: 0.8,
            failure_runtime: 0.8,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 0.4,
            phantom: std::marker::PhantomData,
        }
    }

    fn crack(&self, text: &str) -> Option<String> {
        trace!("Trying hash with text {:?}", text);

        let decoded_text = decode_hash_no_error_handling(text);

        if decoded_text.is_none() {
            debug!("Failed to decode hash because HashDecoder::decode_hash_no_error_handling returned None");
            return None;
        }

        println!("{}", decoded_text.as_ref().unwrap());

        decoded_text
    }
}

fn decode_hash_no_error_handling(text: &str) -> Option<String> {
    // Runs the code to decode hash
    // Doesn't perform error handling

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Data {
        body: std::collections::HashMap<String, Body>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Body {
        plaintext: String,
    }

    let mut data = HashMap::new();
    data.insert("Hash", [&text]);

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://av5b81zg3k.execute-api.us-east-2.amazonaws.com/prod/lookup")
        .json(&data)
        .send()
        .unwrap();

    println!("{resp:?}");

    match resp.status() {
        reqwest::StatusCode::OK => {
            let mut hash_out: Data = resp.json().ok()?;

            if let Some(result) = hash_out.body.remove(text) {
                Some(result.plaintext)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::HashDecoder;
    use crate::decoders::interface::{Crack, Decoder};

    #[test]
    fn successful_decoding() {
        let hash_decoder = Decoder::<HashDecoder>::new();
        let result = hash_decoder
            .crack("098f6bcd4621d373cade4e832627b4f6")
            .unwrap();

        assert_eq!(result, "test");
    }

    #[test]
    fn hash_decode_handles_panics() {
        let hash_decoder = Decoder::<HashDecoder>::new();
        let result = hash_decoder.crack("hello my name is panicky mc panic face!");
        if result.is_some() {
            panic!("Decode_base64 did not return an option with Some<t>.")
        } else {
            // If we get here, the test passed
            // Because the base64_decoder.crack function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }

    #[test]
    fn hash_handle_panic_if_empty_string() {
        let hash_decoder = Decoder::<HashDecoder>::new();
        let result = hash_decoder.crack("");
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn hash_handle_panic_if_emoji() {
        let hash_decoder = Decoder::<HashDecoder>::new();
        let result = hash_decoder.crack("😂");
        if result.is_some() {
            assert_eq!(true, true);
        }
    }
}
