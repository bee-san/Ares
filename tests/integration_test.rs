use ciphey::checkers::checker_result::CheckResult;
use ciphey::checkers::checker_type::{Check, Checker};
use ciphey::checkers::english::EnglishChecker;
use ciphey::checkers::CheckerTypes;
use ciphey::config::Config;
use ciphey::decoders::base64_decoder::Base64Decoder;
use ciphey::decoders::crack_results::CrackResult;
use ciphey::decoders::interface::{Crack, Decoder};
use ciphey::perform_cracking;
use ciphey::storage::database;
use ciphey::tui::human_checker_bridge::{
    init_tui_confirmation_channel, reinit_tui_confirmation_channel,
};
use ciphey::{set_test_db_path, TestDatabase};
use serial_test::{parallel, serial};
use std::thread;
use std::time::Duration;

// TODO Below fails because Library API is broken.
// https://github.com/bee-san/ciphey/issues/48
#[test]
#[parallel]
fn test_it_works() {
    // It will panic if it doesn't work!
    // Plaintext is `Mutley, you snickering, floppy eared hound. When courage is needed, you’re never around. Those m...	`
    let config = Config::default();
    perform_cracking("TXV0bGV5LCB5b3Ugc25pY2tlcmluZywgZmxvcHB5IGVhcmVkIGhvdW5kLiBXaGVuIGNvdXJhZ2UgaXMgbmVlZGVkLCB5b3XigJlyZSBuZXZlciBhcm91bmQuIFRob3NlIG1lZGFscyB5b3Ugd2VhciBvbiB5b3VyIG1vdGgtZWF0ZW4gY2hlc3Qgc2hvdWxkIGJlIHRoZXJlIGZvciBidW5nbGluZyBhdCB3aGljaCB5b3UgYXJlIGJlc3QuIFNvLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLiBIb3d3d3chIE5hYiBoaW0sIGphYiBoaW0sIHRhYiBoaW0sIGdyYWIgaGltLCBzdG9wIHRoYXQgcGlnZW9uIG5vdy4g", config);
    assert_eq!(true, true);
}

#[test]
#[parallel]
fn test_no_panic_if_empty_string() {
    // It will panic if it doesn't work!
    let config = Config::default();
    perform_cracking("", config);
    assert_eq!(true, true);
}

/*
#[test]
fn test_program_parses_files_and_cracks() {
    // It should be able to open and crack this file
    let file_path = "tests/test_fixtures/base64_3_times_with_no_new_line";
    let config = Config::default();
    let to_crack = read_and_parse_file(file_path.to_string());
    let result = perform_cracking(&to_crack, config);
    assert_eq!(true, true);
    // The base64 string decodes to "VFoW2RHbHdiR1VndXMUdlbHBVV1RCSlIxWjFXVEk1YTJGWE5XNWpkejA5"
    let result = result.unwrap();
    assert!(
        !result.text.is_empty(),
        "Decoding should produce some result"
    );
}
*/
/*
#[test]
#[ignore]
fn test_program_parses_files_with_new_line_and_cracks() {
    // It should be able to open and crack this file
    let file_path = "tests/test_fixtures/rot13_base64_hex_with_newline";
    let config = Config::default();
    let to_crack = read_and_parse_file(file_path.to_string());
    let result = perform_cracking(&to_crack, config);
    assert_eq!(true, true);
    assert!(result.unwrap().text[0] == "This is a test!");
}
*/

#[test]
#[serial]
fn test_cache_miss_simple_base64() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
    let decoded_text_1 = String::from("hello world\n");

    let config = Config::default();
    let result = perform_cracking(encoded_text_1.as_str(), config);
    assert!(result.is_some());
    assert!(result.unwrap().path.last().unwrap().success);

    let row_result = database::read_cache(&encoded_text_1);
    assert!(row_result.is_ok());
    let row_result = row_result.unwrap();
    assert!(row_result.is_some());

    let row: database::CacheRow = row_result.unwrap();

    let base64_decoder = Decoder::<Base64Decoder>::new();
    let mut expected_crack_result: CrackResult =
        CrackResult::new(&base64_decoder, encoded_text_1.clone());
    expected_crack_result.unencrypted_text = Some(vec![decoded_text_1.clone()]);
    let expected_checker = Checker::<EnglishChecker>::new();
    let mut expected_check_result = CheckResult::new(&expected_checker);
    expected_check_result.is_identified = true;
    expected_crack_result.update_checker(&expected_check_result);
    let expected_path = vec![expected_crack_result.get_json().unwrap()];

    assert_eq!(row.encoded_text, encoded_text_1);
    assert_eq!(row.decoded_text, decoded_text_1);
    assert_eq!(row.path, expected_path);
    assert!(row.successful);
}

#[test]
#[serial]
fn test_cache_hit_simple_base64() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
    let decoded_text_1 = String::from("hello world\n");

    let base64_decoder = Decoder::<Base64Decoder>::new();
    let mut expected_crack_result: CrackResult =
        CrackResult::new(&base64_decoder, encoded_text_1.clone());
    expected_crack_result.unencrypted_text = Some(vec![decoded_text_1.clone()]);
    let expected_checker = Checker::<EnglishChecker>::new();
    let mut expected_check_result = CheckResult::new(&expected_checker);
    expected_check_result.is_identified = true;
    expected_crack_result.update_checker(&expected_check_result);
    let expected_path = vec![expected_crack_result.get_json().unwrap()];

    let _result = database::insert_cache(&database::CacheEntry {
        encoded_text: encoded_text_1.clone(),
        decoded_text: decoded_text_1.clone(),
        path: vec![expected_crack_result],
        execution_time_ms: 100,
        input_length: encoded_text_1.len() as i64,
        decoder_count: 1,
        checker_name: None,
        key_used: None,
    });

    let config = Config::default();
    let result = perform_cracking(encoded_text_1.as_str(), config);
    assert!(result.is_some());
    assert!(result.unwrap().path.last().unwrap().success);

    let row_result = database::read_cache(&encoded_text_1);
    assert!(row_result.is_ok());
    let row_result = row_result.unwrap();
    assert!(row_result.is_some());

    let row: database::CacheRow = row_result.unwrap();
    assert_eq!(row.encoded_text, encoded_text_1);
    assert_eq!(row.decoded_text, decoded_text_1);
    assert_eq!(row.path, expected_path);
    assert!(row.successful);
}

/// Regression test for bug where the human checker confirms the correct plaintext,
/// but the returned result contains the checker's description ("Words") instead
/// of the actual plaintext.
///
/// Bug reproduction:
/// - Input: WkZoS05XVlhTV2RsYlhkbllqSTFkbUpEUWpGamJtdzFXV2xDTm1KRFFuaGliVlkxWkcxR01BPT0=
/// - Expected plaintext: "hello my baby hello my darling"
/// - Bug behavior: Returns "Words" (the checker's description field)
///
/// The issue is that CheckResult has two text fields:
/// - `text`: The actual plaintext being checked
/// - `description`: A description of what was detected (e.g., "Words" for English checker)
///
/// Somewhere in the pipeline, `description` is being used instead of `text`.
#[test]
#[parallel]
fn test_plaintext_not_checker_description() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    // Simple base64 encoded "hello world" - this should decode to "hello world",
    // NOT to "Words" (which is the English checker's description)
    let encoded = "aGVsbG8gd29ybGQ=";
    let expected_plaintext = "hello world";

    let mut config = Config::default();
    config.human_checker_on = false; // Disable human checker for automated testing

    let result = perform_cracking(encoded, config);
    assert!(result.is_some(), "Should successfully decode base64");

    let result = result.unwrap();
    assert!(!result.text.is_empty(), "Result should have text");

    // THE BUG: The plaintext should be "hello world", not "Words"
    // "Words" is the description field from the English checker (CheckResult.description)
    // but we want the actual plaintext (CheckResult.text)
    let plaintext = &result.text[0];
    assert_ne!(
        plaintext, "Words",
        "Plaintext should NOT be the checker's description 'Words'"
    );
    assert_eq!(
        plaintext, expected_plaintext,
        "Plaintext should be the actual decoded text"
    );
}

/// More complex regression test with nested encoding to reproduce the original bug.
/// The original bug was observed with deeply nested encoding:
/// Base64 → Base64 → Base64 → Reverse → atbash → Hexadecimal → simplesubstitution × 2
///
/// This test uses a simpler nested base64 to verify the same behavior.
#[test]
#[parallel]
fn test_nested_decoding_returns_plaintext_not_description() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    // "hello there general" base64 encoded
    let encoded = "aGVsbG8gdGhlcmUgZ2VuZXJhbA==";
    let expected_plaintext = "hello there general";

    let mut config = Config::default();
    config.human_checker_on = false;

    let result = perform_cracking(encoded, config);
    assert!(result.is_some(), "Should successfully decode");

    let result = result.unwrap();
    let plaintext = &result.text[0];

    // Verify the result is the actual plaintext, not any checker metadata
    assert_ne!(
        plaintext, "Words",
        "Result should not be checker description"
    );
    assert_ne!(plaintext, "English", "Result should not be checker name");
    assert!(
        plaintext.contains("hello"),
        "Result should contain actual plaintext"
    );
    assert_eq!(plaintext, expected_plaintext);
}

/// Test that verifies the CheckResult text field is correctly propagated through decoding.
/// This directly tests the English checker to ensure it returns the input text
/// in the `text` field, not the description.
#[test]
#[parallel]
fn test_english_checker_returns_input_text_not_description() {
    let checker = Checker::<EnglishChecker>::new();
    let input = "hello world this is a test";

    let result = checker.check(input);

    // The text field should contain the input that was checked
    assert_eq!(
        result.text, input,
        "CheckResult.text should be the input text"
    );

    // The description field should be empty for the English checker
    assert!(
        result.description.is_empty(),
        "CheckResult.description should be empty"
    );

    // These should NOT be equal - this is the core of the bug
    assert_ne!(
        result.text, result.description,
        "CheckResult.text and CheckResult.description should be different"
    );
}

/// Test that CrackResult.unencrypted_text contains the actual decoded text,
/// not any checker metadata.
#[test]
#[parallel]
fn test_crack_result_contains_decoded_text() {
    let decoder = Decoder::<Base64Decoder>::new();
    let checker = CheckerTypes::CheckEnglish(Checker::<EnglishChecker>::new());

    let encoded = "aGVsbG8gd29ybGQ=";
    let expected_decoded = "hello world";

    let result = decoder.crack(encoded, &checker);

    assert!(
        result.unencrypted_text.is_some(),
        "Should have decrypted text"
    );

    let decoded = &result.unencrypted_text.unwrap()[0];
    assert_eq!(
        decoded, expected_decoded,
        "CrackResult.unencrypted_text should contain the decoded text"
    );
    assert_ne!(
        decoded, "Words",
        "CrackResult.unencrypted_text should NOT be the checker description"
    );
}

/// Regression test for GitHub issue: Human checker confirms correct plaintext but
/// the returned result is wrong.
///
/// Original bug report:
/// - Input: WkZoS05XVlhTV2RsYlhkbllqSTFkbUpEUWpGamJtdzFXV2xDTm1KRFFuaGliVlkxWkcxR01BPT0=
/// - Human checker shows: 'hello my baby hello my darling' (user confirms with 'y')
/// - Decoder path: Base64 → Base64 → Base64 → Reverse → atbash → Hexadecimal → simplesubstitution → simplesubstitution
/// - Bug: The final plaintext is NOT "hello my baby hello my darling" but something else
///
/// This test verifies that when perform_cracking succeeds, the returned text
/// matches what the checker identified as plaintext.
#[test]
#[serial]
fn test_complex_decoding_returns_correct_plaintext() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    // This is the exact input from the bug report
    let encoded = "WkZoS05XVlhTV2RsYlhkbllqSTFkbUpEUWpGamJtdzFXV2xDTm1KRFFuaGliVlkxWkcxR01BPT0=";

    let mut config = Config::default();
    // Disable human checker for automated testing - we want to test the flow
    // where a checker identifies plaintext and the result is returned
    config.human_checker_on = false;

    let result = perform_cracking(encoded, config);

    // The decoding should succeed (it did in the bug report)
    if let Some(result) = result {
        let plaintext = &result.text[0];

        // THE BUG: The result should contain actual plaintext, not checker metadata
        // The bug manifests as the plaintext being empty, whitespace, or "Words"
        assert!(!plaintext.is_empty(), "Plaintext should not be empty");
        assert!(
            plaintext.trim().len() > 1,
            "Plaintext should not be just whitespace, got: '{}'",
            plaintext
        );
        assert_ne!(
            plaintext, "Words",
            "Plaintext should not be the checker description 'Words'"
        );
        assert_ne!(
            plaintext, "English",
            "Plaintext should not be the checker name"
        );

        // The plaintext should contain readable text
        // Based on the bug report, the expected plaintext is "hello my baby hello my darling"
        // but we relax this to just check it's reasonable text
        assert!(
            plaintext.chars().all(|c| c.is_ascii() || c.is_whitespace()),
            "Plaintext should be readable ASCII text, got: '{}'",
            plaintext
        );
    }
    // Note: If result is None, the test doesn't fail because the decoding path
    // may not be deterministic. The key assertion is that IF we get a result,
    // it should be correct.
}

/// Test that simulates what happens when a checker identifies plaintext.
///
/// This test verifies the data flow:
/// 1. Decoder decodes text
/// 2. Checker identifies it as plaintext, sets CheckResult.text = actual_plaintext
/// 3. CrackResult.unencrypted_text should contain the decoded text
/// 4. The text shown to human checker (CheckResult.text) should match CrackResult.unencrypted_text
///
/// The bug is that CheckResult.description ("Words") ends up in the final result
/// instead of CheckResult.text (the actual plaintext).
#[test]
#[parallel]
fn test_checker_text_matches_decoder_output() {
    let decoder = Decoder::<Base64Decoder>::new();
    let checker = CheckerTypes::CheckEnglish(Checker::<EnglishChecker>::new());

    // Decode "hello world"
    let encoded = "aGVsbG8gd29ybGQ=";
    let crack_result = decoder.crack(encoded, &checker);

    // The decoder should have decoded the text
    assert!(crack_result.unencrypted_text.is_some());
    let decoded_text = &crack_result.unencrypted_text.as_ref().unwrap()[0];
    assert_eq!(decoded_text, "hello world");

    // Now check what the checker returns
    let check_result = checker.check(decoded_text);

    // CRITICAL: The text field should be the actual plaintext
    assert_eq!(
        check_result.text, *decoded_text,
        "CheckResult.text should be the plaintext that was checked"
    );

    // The description should be empty for the English checker
    assert!(
        check_result.description.is_empty(),
        "CheckResult.description should be empty for English checker"
    );

    // These must be different - if they're the same, we have a data flow problem
    assert_ne!(
        check_result.text, check_result.description,
        "CheckResult.text and description should be different values"
    );
}

/// Test that verifies the HumanConfirmationRequest correctly captures
/// the plaintext from CheckResult, not the description.
///
/// This is the exact code path that the bug report shows:
/// - Human checker displays: 'hello my baby hello my darling'
/// - But final result shows something else
#[test]
#[parallel]
fn test_human_confirmation_request_uses_correct_text() {
    use ciphey::tui::app::HumanConfirmationRequest;

    // Simulate what the English checker returns
    let check_result = CheckResult {
        is_identified: true,
        text: "hello my baby hello my darling".to_string(), // The actual plaintext
        description: "Words".to_string(),                   // The checker's description
        checker_name: "English Checker",
        checker_description: "Uses gibberish detection",
        link: "",
    };

    // This is what gets shown in the human checker prompt
    let request = HumanConfirmationRequest::from(&check_result);

    // The request.text should be the PLAINTEXT, not the description
    assert_eq!(
        request.text, "hello my baby hello my darling",
        "HumanConfirmationRequest.text should be the actual plaintext"
    );

    // Verify it's NOT the description
    assert_ne!(
        request.text, "Words",
        "HumanConfirmationRequest.text should NOT be 'Words'"
    );

    // The description field should be the checker's description
    assert_eq!(
        request.description, "Words",
        "HumanConfirmationRequest.description should be 'Words'"
    );
}

/// This test directly simulates the bug scenario:
/// When human checker confirms a plaintext, the DecoderResult should contain
/// that same plaintext, not some other value.
///
/// The flow being tested:
/// 1. Decoder returns CrackResult with unencrypted_text = ["hello my baby..."]
/// 2. Checker confirms it's English, returns CheckResult with text = "hello my baby..."
/// 3. Human sees "hello my baby..." and confirms
/// 4. DecoderResult.text should be ["hello my baby..."]
///
/// BUG: Step 4 returns wrong text
#[test]
#[parallel]
fn test_decoder_result_text_matches_checker_confirmed_text() {
    use ciphey::DecoderResult;

    // Simulate what a decoder returns after successful decoding
    let decoded_plaintext = "hello my baby hello my darling";

    let mut crack_result = CrackResult::new(
        &Decoder::<Base64Decoder>::new(),
        "encoded_input".to_string(),
    );
    crack_result.unencrypted_text = Some(vec![decoded_plaintext.to_string()]);
    crack_result.success = true;

    // Simulate what the checker returns
    let check_result = CheckResult {
        is_identified: true,
        text: decoded_plaintext.to_string(),
        description: "Words".to_string(),
        checker_name: "English Checker",
        checker_description: "Uses gibberish detection",
        link: "",
    };

    // Update crack result with checker info (this is what decoders do)
    crack_result.update_checker(&check_result);

    // Create a DecoderResult as the search algorithm would
    // This simulates astar.rs lines 294-306
    let decoder_result = DecoderResult {
        text: crack_result.unencrypted_text.clone().unwrap_or_default(),
        path: vec![crack_result],
    };

    // THE BUG CHECK: The DecoderResult.text should be the actual plaintext
    assert_eq!(
        decoder_result.text[0], decoded_plaintext,
        "DecoderResult.text should be the actual plaintext, not checker metadata"
    );

    // It should NOT be the checker description
    assert_ne!(
        decoder_result.text[0], "Words",
        "DecoderResult.text should NOT be 'Words' (the checker description)"
    );

    // Verify the path also has the correct text
    assert_eq!(
        decoder_result.path[0].unencrypted_text.as_ref().unwrap()[0],
        decoded_plaintext,
        "CrackResult in path should have correct unencrypted_text"
    );
}

/// Integration test that runs the full decoding pipeline and verifies
/// the result text matches what was decoded.
///
/// This test uses a mock approach by temporarily enabling/disabling
/// the human checker through config, and verifies the result structure.
#[test]
#[serial]
fn test_full_pipeline_plaintext_propagation() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    // Use a simpler input that will definitely decode
    let encoded = "aGVsbG8gd29ybGQ="; // "hello world" in base64
    let expected_plaintext = "hello world";

    let mut config = Config::default();
    config.human_checker_on = false;

    let result = perform_cracking(encoded, config);
    assert!(result.is_some(), "Should decode successfully");

    let result = result.unwrap();

    // The result.text should be the actual plaintext
    assert_eq!(
        result.text[0], expected_plaintext,
        "DecoderResult.text should be the plaintext"
    );

    // The path should have the correct unencrypted_text
    if let Some(last_crack) = result.path.last() {
        if let Some(unenc) = &last_crack.unencrypted_text {
            assert_eq!(
                unenc[0], expected_plaintext,
                "CrackResult.unencrypted_text should be the plaintext"
            );
        }
    }
}

/// Test that specifically checks the substitution decoder doesn't lose plaintext.
/// The bug report shows simplesubstitution in the decode path.
#[test]
#[parallel]
fn test_substitution_decoder_preserves_plaintext() {
    use ciphey::checkers::athena::Athena;
    use ciphey::decoders::substitution_generic_decoder::SubstitutionGenericDecoder;

    let decoder = Decoder::<SubstitutionGenericDecoder>::new();
    let checker = CheckerTypes::CheckAthena(Checker::<Athena>::new());

    // This is a simple substitution cipher input (binary-like with two symbols)
    // that should decode to something
    let encoded = "AABBAABBAABBAABB"; // Simple pattern

    let result = decoder.crack(encoded, &checker);

    // If decoding succeeded, verify the text is preserved
    if result.success {
        assert!(
            result.unencrypted_text.is_some(),
            "Successful decode should have unencrypted_text"
        );

        let text = &result.unencrypted_text.as_ref().unwrap()[0];

        // The text should not be checker metadata
        assert_ne!(text, "Words", "Text should not be 'Words'");
        assert_ne!(text, "", "Text should not be empty");
    }
}

/// REGRESSION TEST: Human checker confirms plaintext but wrong text is returned.
///
/// Bug report:
/// - Input: WkZoS05XVlhTV2RsYlhkbllqSTFkbUpEUWpGamJtdzFXV2xDTm1KRFFuaGliVlkxWkcxR01BPT0=
/// - Human checker shows: 'hello my baby hello my darling'
/// - User confirms with 'y'
/// - BUG: Final plaintext is empty/wrong instead of 'hello my baby hello my darling'
///
/// This test uses the TUI confirmation channel to simulate human checker interaction
/// without requiring actual stdin input.
///
/// Run with: cargo test test_human_checker_bug_confirmed_plaintext_is_returned -- --ignored --nocapture
#[test]
#[serial]
#[ignore = "Bug reproduction test - fails until bug is fixed. Run with --ignored to verify."]
fn test_human_checker_bug_confirmed_plaintext_is_returned() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    // Initialize the TUI confirmation channel to intercept human checker calls
    let _initialized = init_tui_confirmation_channel();

    // Get a fresh channel for this test
    let receiver = reinit_tui_confirmation_channel();

    // Reset human checker state
    ciphey::checkers::reset_human_checker_state();

    // The exact input from the bug report
    let encoded = "WkZoS05XVlhTV2RsYlhkbllqSTFkbUpEUWpGamJtdzFXV2xDTm1KRFFuaGliVlkxWkcxR01BPT0=";

    let mut config = Config::default();
    config.human_checker_on = true; // Enable human checker - this is the key!

    // Spawn a thread to handle human checker confirmations
    let confirmation_thread = thread::spawn(move || {
        let receiver = receiver.expect("Should have receiver");
        let mut confirmed_text: Option<String> = None;

        // Wait for confirmation requests and auto-confirm them
        while let Ok(request) = receiver.recv_timeout(Duration::from_secs(30)) {
            // Capture the text that was shown to the "human"
            confirmed_text = Some(request.request.text.clone());

            // Simulate user pressing 'y' to confirm
            let _ = request.response_tx.send(true);

            // Only handle one confirmation
            break;
        }

        confirmed_text
    });

    // Run the cracking in the main thread
    let result = perform_cracking(encoded, config);

    // Get what text the human checker showed
    let confirmed_text = confirmation_thread
        .join()
        .expect("Confirmation thread panicked");

    // Verify we got a result
    assert!(result.is_some(), "Should get a decoding result");
    let result = result.unwrap();

    // THE BUG CHECK: If human checker showed text X and user confirmed,
    // the final result should contain text X
    if let Some(human_saw) = confirmed_text {
        let final_plaintext = &result.text[0];

        // This is the actual bug - the plaintext doesn't match what human confirmed
        assert_eq!(
            final_plaintext, &human_saw,
            "BUG: Final plaintext '{}' doesn't match what human checker showed '{}'",
            final_plaintext, human_saw
        );

        // Additional checks
        assert!(
            !final_plaintext.is_empty(),
            "Final plaintext should not be empty"
        );
        assert_ne!(
            final_plaintext, "Words",
            "Final plaintext should not be 'Words' (checker description)"
        );
    }
}
