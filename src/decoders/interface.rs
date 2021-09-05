///! The Interface defines what the struct for each decoder looks like
// TODO rename this file
pub struct Decoder {
    pub name: &'static str,
    /// A description, you can take the first line from Wikipedia
    /// Sometimes our decoders do not exist on Wikipedia so we write our own.
    pub description: &'static str,
    /// Wikipedia Link
    pub link: &'static str,
    pub tags: Vec<&'static str>,
    /// We get expectedRuntime this by bench marking the code
    pub expected_runtime: f32,
    /// We get popularity by eye-balling it or using the API's data
    pub popularity: f32,
    /// Expected success is calculated during cracking
    /// Generally this can be ignored and set to 1.0
    pub expected_success: f32,
    /// failure_runtime is the absolute worst case
    /// Expected is how long we expect, if it fails completely
    /// This is how long it'll take to fail.
    pub failure_runtime: f32,
    // normalised_entropy is the range of entropy for this
    // So base64's normalised entropy might be between 2.5 and 3
    // This allows us to decide whether it's worth decoding
    // If current text has entropy 9, it's unlikey to be base64
    pub normalised_entropy: Vec<f32>,
}

/// All decoders will share the same Crack trait
/// Which let's us put them into a vector and iterate over them,
/// Running `.crack()` on each of them.
/// Relevant docs: https://docs.rs/crack/0.3.0/crack/trait.Crack.html
pub trait Crack {
    fn crack(&self, text: &str) -> Option<String>;
}
