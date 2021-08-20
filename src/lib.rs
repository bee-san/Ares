//! Ares is an automatic decoding and cracking tool.

mod decoders;
use decoders::base64_decoder::decode_base64;

pub fn crack(text: &str) {
    println!("{:?}", decode_base64(text).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
