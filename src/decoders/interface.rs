///! This is what each decoder looks like
///! With what they must implement

pub struct Decoder {
    name: String,
    tags: Vec<String>,
    expectedRuntime: u32,
    popularity: i32,
    expectedSuccess: u32,
}

pub impl Decoder {
    fn new() -> Self {
        Decoder {
            name: String,
            tags: Vec<String>,
            /// We get expectedRuntime this by bench marking the code
            expected_runtime: u32,
            /// We get popularity by eye-balling it or using the API's data
            popularity: u16,
            /// Expected success is calculated during cracking
            /// Generally this can be ignored and set to 1.0
            expected_success: u32,
            /// failure_runtime is the absolute worst case
            /// Expected is how long we expect, if it fails completely
            /// This is how long it'll take to fail.
            failure_runtime: u32,
            // normalised_entropy is the range of entropy for this
            // So base64's normalised entropy might be between 2.5 and 3
            // This allows us to decide whether it's worth decoding
            normalised_entropy: Vec<u32, u32>,
        }
    }
}

pub trait Decoder {
    // All decoders should have a crack() method
    // That does the decoding and returns the result as a string
    fn crack(&self) -> String;
}