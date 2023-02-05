use crate::checkers::checker_result::CheckResult;
use crate::cli_pretty_printing::human_checker_check;
use crate::config::get_config;
use crate::timer;
use text_io::read;

/// The Human Checker asks humans if the expected plaintext is real plaintext
/// We can use all the automated checkers in the world, but sometimes they get false positives
/// Humans have the last say.
/// TODO: Add a way to specify a list of checkers to use in the library. This checker is not library friendly!
// compile this if we are not running tests
pub fn human_checker(input: &CheckResult) -> bool {
    timer::pause();
    // wait instead of get so it waits for config being set
    let config = get_config();
    // We still call human checker, just if config is false we return True
    if !config.human_checker_on || config.api_mode {
        return true;
    }
    human_checker_check(&input.description, &input.text);

    let reply: String = read!("{}\n");
    let result = reply.to_ascii_lowercase().starts_with('y');
    if !result {
        timer::resume();
        return false;
    }
    true
}
