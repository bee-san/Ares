//! Ares is an automatic decoding and cracking tool.

mod decoders;
mod filtration_system;
use crate::decoders::base64_decoder::{Base64Decoder};

/// The main function to call which performs the cracking.
/// ```rust
/// use ares::perform_cracking;
/// perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu");
/// assert!(true, true)
/// ```
pub fn perform_cracking(text: &str) {
    let base64_decoder = Base64Decoder::new();
    println!("{:?}", base64_decoder.crack(text).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
