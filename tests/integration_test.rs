use ciphey::checkers::checker_result::CheckResult;
use ciphey::checkers::checker_type::{Check, Checker};
use ciphey::checkers::english::EnglishChecker;
use ciphey::cli::read_and_parse_file;
use ciphey::config::Config;
use ciphey::decoders::base64_decoder::Base64Decoder;
use ciphey::decoders::crack_results::CrackResult;
use ciphey::decoders::interface::{Crack, Decoder};
use ciphey::perform_cracking;
use ciphey::storage::database;
use serial_test::serial;

// TODO Below fails because Library API is broken.
// https://github.com/bee-san/ciphey/issues/48
#[test]
fn test_it_works() {
    // It will panic if it doesn't work!
    // Plaintext is `Mutley, you snickering, floppy eared hound. When courage is needed, you’re never around. Those m...	`
    let config = Config::default();
    perform_cracking("TXV0bGV5LCB5b3Ugc25pY2tlcmluZywgZmxvcHB5IGVhcmVkIGhvdW5kLiBXaGVuIGNvdXJhZ2UgaXMgbmVlZGVkLCB5b3XigJlyZSBuZXZlciBhcm91bmQuIFRob3NlIG1lZGFscyB5b3Ugd2VhciBvbiB5b3VyIG1vdGgtZWF0ZW4gY2hlc3Qgc2hvdWxkIGJlIHRoZXJlIGZvciBidW5nbGluZyBhdCB3aGljaCB5b3UgYXJlIGJlc3QuIFNvLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLCBzdG9wIHRoYXQgcGlnZW9uLiBIb3d3d3chIE5hYiBoaW0sIGphYiBoaW0sIHRhYiBoaW0sIGdyYWIgaGltLCBzdG9wIHRoYXQgcGlnZW9uIG5vdy4g", config);
    assert_eq!(true, true);
}

#[test]
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

/// Gets the test directory path
fn get_test_dir_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push("Ares");
    path.push("Test");
    path
}

/// Sets the global database path
fn set_test_db_path() {
    let mut path = get_test_dir_path();
    std::fs::create_dir_all(&path).expect("Could not create Ares directory");
    path.push("database.sqlite");
    let _ = database::DB_PATH.set(Some(path));
}

struct TestDatabase {
    pub path: std::path::PathBuf,
}

impl Default for TestDatabase {
    fn default() -> Self {
        TestDatabase {
            path: get_test_dir_path(),
        }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let mut db_file_path = self.path.as_path().to_path_buf();
        db_file_path.push("database.sqlite");
        let _ = std::fs::remove_file(&db_file_path);
        let _ = std::fs::remove_dir(&self.path);
    }
}

#[test]
#[serial]
fn test_cache_simple_base64() {
    let _test_db = TestDatabase::default();
    set_test_db_path();

    let encoded_text_1 = String::from("aGVsbG8gd29ybGQK");
    let decoded_text_1 = String::from("hello world\n");

    let config = Config::default();
    let result = perform_cracking(encoded_text_1.as_str(), config);
    assert!(result.is_some());
    assert_eq!(result.unwrap().path.last().unwrap().success, true);

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
    assert_eq!(row.successful, true);
}
