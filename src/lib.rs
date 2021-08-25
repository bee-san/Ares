//! Ares is an automatic decoding and cracking tool.

mod decoders;
mod filtration_system;
use crate::filtration_system::filter_and_get_decoders;

/// The main function to call which performs the cracking.
/// ```rust
/// use ares::perform_cracking;
/// perform_cracking("VGhlIG1haW4gZnVuY3Rpb24gdG8gY2FsbCB3aGljaCBwZXJmb3JtcyB0aGUgY3JhY2tpbmcu");
/// assert!(true, true)
/// ```
pub fn perform_cracking(text: &str) -> Vec<Option<String>> {
    let decoders = filter_and_get_decoders();
    let y = decoders.run(text);
    println!("{:?}", y);
    // TODO should it return here or later?
    y
}

#[cfg(test)]
mod tests {
    use super::perform_cracking;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_perform_cracking_returns() {
        perform_cracking("SGVscCBJIG5lZWQgc29tZWJvZHkh");
    }
}
