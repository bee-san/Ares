///! The Interface defines what the struct for each decoder looks like
//TODO: rename this file
pub struct Decoder<Type> {
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
    // we don't use the Type, so we use PhantomData to mark it!
    pub phantom: std::marker::PhantomData<Type>,
}

/// All decoders will share the same Crack trait
/// Which let's us put them into a vector and iterate over them,
/// Running `.crack()` on each of them.
/// Relevant docs: https://docs.rs/crack/0.3.0/crack/trait.Crack.html
pub trait Crack {
    fn new() -> Self
    where
        Self: Sized;
    fn crack(&self, text: &str) -> Option<String>;
}

/// Returns a boolean of True if the string is successfully changed
/// So empty strings fail, but non-empty strings succeed
/// and only if the string is different from the original text.
pub fn check_string_success(decoded_text: &str, original_text: &str) -> bool {
    if decoded_text.is_empty() {
        return false;
    } else if decoded_text != original_text {
        return true;
    }
    false
}
