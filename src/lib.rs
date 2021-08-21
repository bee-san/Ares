//! Ares is an automatic decoding and cracking tool.

mod decoders;
use crate::decoders::base64_decoder::{Base64Decoder};

pub fn perform_cracking(text: &str) {
    let base64_decoder = Base64Decoder::new();
    println!("{:?}", base64_decoder.Crack(text).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
