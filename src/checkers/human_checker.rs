use crate::checkers::checker_result::CheckResult;

#[cfg(not(test))]
use text_io::read;

/// The Human Checker asks humans if the expected plaintext is real plaintext
/// We can use all the automated checkers in the world, but sometimes they get false positives
/// Humans have the last say.
/// TODO: Add a way to specify a list of checkers to use in the library. This checker is not library friendly!
// compile this if we are not running tests
#[cfg(not(test))]
pub fn human_checker(input: &CheckResult) -> bool {
    let output_string = format!(
        "Is the plaintext '{}' which is {}. [Y/n]? ",
        input.text, input.description
    );
    // print output_string and ask user to input yes or no
    println!("{}", output_string);
    let reply: String = read!("{}\n");
    if reply.starts_with('n') {
        return false;
    }
    true
}

// use this human_checker for tests
#[cfg(test)]
pub fn human_checker(_input: &CheckResult) -> bool {
    true
}
