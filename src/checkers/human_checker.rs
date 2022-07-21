use text_io::read;

use crate::checkers::checker_object::{CheckObject};

#[allow(dead_code)]
pub fn human_checker(input: CheckObject) -> bool {
    let output_string = format!("Is the plaintext '{}' which is {}. [Y/n]? ", input.text, input.description);
    // print output_string and ask user to input yes or no
    println!("{}", output_string);
    let reply: String = read!("{}\n");
    if reply.starts_with("n") {
        return false;
    }
    true
}