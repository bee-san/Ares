use ciphey::checkers::{
    athena::Athena,
    checker_type::{Check, Checker},
    CheckerTypes,
};
use ciphey::decoders::{
    base64_decoder::Base64Decoder,
    caesar_decoder::CaesarDecoder,
    interface::{Crack, Decoder},
};

#[test]
fn test_base64_then_caesar() {
    let checker = CheckerTypes::CheckAthena(Checker::<Athena>::new());

    // First decode base64
    let base64_decoder = Decoder::<Base64Decoder>::new();
    let base64_result = base64_decoder.crack("dXJ5eWIgamJleXEhIHpsIGFuenIgdmYgbmhnaHph", &checker);

    println!("Base64 result success: {}", base64_result.success);
    println!("Base64 result: {:?}", base64_result.unencrypted_text);

    assert!(base64_result.unencrypted_text.is_some());
    let decoded_texts = base64_result.unencrypted_text.unwrap();
    assert!(!decoded_texts.is_empty());

    let intermediate_text = &decoded_texts[0];
    println!("Base64 decoded to: {}", intermediate_text);
    assert_eq!(intermediate_text, "uryyb jbeyq! zl anzr vf nhghza");

    // Now try caesar on the decoded text
    let caesar_decoder = Decoder::<CaesarDecoder>::new();
    let caesar_result = caesar_decoder.crack(intermediate_text, &checker);

    println!("Caesar result success: {}", caesar_result.success);
    println!("Caesar result: {:?}", caesar_result.unencrypted_text);

    assert!(caesar_result.success);
    assert!(caesar_result.unencrypted_text.is_some());

    let final_texts = caesar_result.unencrypted_text.unwrap();
    assert!(!final_texts.is_empty());
    println!("Final plaintext: {}", &final_texts[0]);
    assert_eq!(final_texts[0], "hello world! my name is autumn");
}
