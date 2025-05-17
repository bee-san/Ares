use crate::checkers::checker_result::CheckResult;
use crate::cli_pretty_printing::human_checker_check;
use crate::config::get_config;
use crate::storage::database;
use crate::{cli_pretty_printing, timer};
use dashmap::DashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use text_io::read;

static SEEN_PROMPTS: OnceLock<DashSet<String>> = OnceLock::new();
// if human checker is called, we set this to true
// so we dont call it again
static HUMAN_CONFIRMED: AtomicBool = AtomicBool::new(false);

fn get_seen_prompts() -> &'static DashSet<String> {
    SEEN_PROMPTS.get_or_init(DashSet::new)
}

/// The Human Checker asks humans if the expected plaintext is real plaintext
/// We can use all the automated checkers in the world, but sometimes they get false positives
/// Humans have the last say.
/// TODO: Add a way to specify a list of checkers to use in the library. This checker is not library friendly!
// compile this if we are not running tests
pub fn human_checker(input: &CheckResult) -> bool {
    // Check if a human has already confirmed a result
    if HUMAN_CONFIRMED.load(Ordering::Acquire) {
        return true;
    }
    timer::pause();
    // wait instead of get so it waits for config being set
    let config = get_config();
    // We still call human checker, just if config is false we return True
    if !config.human_checker_on || config.api_mode {
        timer::resume();
        return true;
    }

    // Check if we've already prompted for this text
    let prompt_key = format!("{}{}", input.description, input.text);
    if !get_seen_prompts().insert(prompt_key) {
        return true; // Return true to allow the search to continue
    }
    human_checker_check(&input.description, &input.text);

    let reply: String = read!("{}\n");
    cli_pretty_printing::success(&format!("DEBUG: Human checker received reply: '{}'", reply));
    let result = reply.to_ascii_lowercase().starts_with('y');
    // If the user confirmed, set the atomic boolean to true
    if result {
        HUMAN_CONFIRMED.store(true, Ordering::Release);
        cli_pretty_printing::success(
            "DEBUG: Human confirmed a result, future checks will be skipped",
        );
    }
    timer::resume();

    cli_pretty_printing::success(&format!("DEBUG: Human checker returning: {}", result));

    if !result {
        let fd_result = database::insert_human_rejection(uuid::Uuid::new_v4(), &input.text, input);
        match fd_result {
            Ok(_) => (),
            Err(e) => {
                cli_pretty_printing::warning(&format!(
                    "DEBUG: Failed to write human checker rejection due to error: {}",
                    e
                ));
            }
        }
        return false;
    }
    true
}
