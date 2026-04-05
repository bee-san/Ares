use ciphey::checkers::{
    athena::Athena,
    checker_type::{Check, Checker},
    wordlist::WordlistChecker,
    CheckerTypes,
};
use ciphey::decoders::{
    base32_decoder::Base32Decoder,
    base64_decoder::Base64Decoder,
    caesar_decoder::CaesarDecoder,
    hexadecimal_decoder::HexadecimalDecoder,
    interface::{Crack, Decoder},
    morse_code::MorseCodeDecoder,
    rot47_decoder::ROT47Decoder,
};

fn get_athena_checker() -> CheckerTypes {
    let athena_checker = Checker::<Athena>::new();
    CheckerTypes::CheckAthena(athena_checker)
}

fn assert_decoder_output<T>(encoded: &str, expected: &str)
where
    Decoder<T>: Crack,
{
    let result = Decoder::<T>::new().crack(encoded, &get_athena_checker());
    let decoded = result
        .unencrypted_text
        .unwrap_or_else(|| panic!("failed to decode CTF sample: {encoded}"));

    assert_eq!(decoded[0], expected);
}

fn assert_decoder_candidates_contain<T>(encoded: &str, expected: &str)
where
    Decoder<T>: Crack,
{
    let checker = CheckerTypes::CheckWordlist(Checker::<WordlistChecker>::new());
    let result = Decoder::<T>::new().crack(encoded, &checker);
    let decoded = result
        .unencrypted_text
        .unwrap_or_else(|| panic!("failed to decode CTF sample: {encoded}"));

    assert!(
        decoded.iter().any(|candidate| candidate == expected),
        "expected candidate not found for CTF sample: {encoded}",
    );
}

#[test]
fn tryhackme_c4ptur3_th3_fl4g_examples_decode() {
    // Source:
    // https://medium.com/@sunjid-ahmed/c4ptur3-th3-fl4g-tryhackme-walkthrough-2d76930adb2a
    assert_decoder_output::<Base64Decoder>(
        "RWFjaCBCYXNlNjQgZGlnaXQgcmVwcmVzZW50cyBleGFjdGx5IDYgYml0cyBvZiBkYXRhLg==",
        "Each Base64 digit represents exactly 6 bits of data.",
    );
    assert_decoder_output::<HexadecimalDecoder>(
        "68 65 78 61 64 65 63 69 6d 61 6c 20 6f 72 20 62 61 73 65 31 36 3f",
        "hexadecimal or base16?",
    );
    assert_decoder_output::<Base32Decoder>(
        "MJQXGZJTGIQGS4ZAON2XAZLSEBRW63LNN5XCA2LOEBBVIRRHOM======",
        "base32 is super common in CTF's",
    );
    assert_decoder_output::<CaesarDecoder>("Ebgngr zr 13 cynprf!", "Rotate me 13 places!");
    assert_decoder_candidates_contain::<ROT47Decoder>(
        "*@F DA:? >6 C:89E C@F?5 323J C:89E C@F?5 Wcf E:>6DX",
        "You spin me right round baby right round (47 times)",
    );
    assert_decoder_output::<MorseCodeDecoder>(
        "- . .-.. . -.-. --- -- -- ..- -. .. -.-. .- - .. --- -.\n. -. -.-. --- -.. .. -. --.",
        "TELECOMMUNICATION ENCODING",
    );
}

#[test]
fn hackthebox_invite_examples_decode() {
    // Source:
    // https://www.aldeid.com/wiki/HackTheBox-Invite
    assert_decoder_output::<CaesarDecoder>(
        "Va beqre gb trarengr gur vaivgr pbqr, znxr n CBFG erdhrfg gb /ncv/vaivgr/trarengr",
        "In order to generate the invite code, make a POST request to /api/invite/generate",
    );
    assert_decoder_output::<Base64Decoder>(
        "RURWREktWUZIRFMtWEZCV0MtRFNIV0MtR0VKWEw=",
        "EDVDI-YFHDS-XFBWC-DSHWC-GEJXL",
    );
}
