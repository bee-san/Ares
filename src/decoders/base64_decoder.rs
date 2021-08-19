///! Decode a base64 string
///! Performs error handling and returns a string
///! Call decode_base64 to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.

use log::{trace};


fn decode_base64_no_error_handling(text: &str) -> Result<String, base64::DecodeError> {
    // Runs the code to decode base64
    // Doesn't perform error handling, call from_base64
    let bytes = base64::decode(text)?;
    let ascii_string = String::from_utf8(bytes).unwrap();
    Ok(ascii_string)
}

pub fn decode_base64(text: &str) -> Option<String> {
    trace!("Trying Base64 with text {:?}", text);
    let result = decode_base64_no_error_handling(text);
    match result {
        Ok(x) => Some(x),
        Err(_) => {
            trace!("Failed to decode base64.");
            None
        }
    }
    /*
    if result.is_err() {
        
    }
    result.unwrap()*/
}

#[cfg(test)]
mod tests {
    use crate::base64_decoder::{decode_base64};

    #[test]
    fn it_works() {
        let _result = decode_base64("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn successful_decoding(){
        let result = decode_base64("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn base64_decode_empty_string(){
        let result = decode_base64("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn base64_decode_handles_panics() {
        let result = decode_base64("hello my name is panicky mc panic face!");
        if result.is_some() {
            panic!("Decode_base64 did not return an option with Some<t>.")
            
        }
        else {
            // If we get here, the test passed
            // Because the decode_base64 function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }
}