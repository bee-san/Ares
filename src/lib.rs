//! Ares is an automatic decoding and cracking tool.

#[path = "decoders/base64_decoder.rs"] mod base64_decoder;

pub fn crack() {
    println!("Test");
    base64_decoder::decode_base64("dGVzdA==");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
