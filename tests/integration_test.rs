use ares::config::Config;
use ares::perform_cracking;

// TODO Below fails because Library API is broken.
// https://github.com/bee-san/Ares/issues/48
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
