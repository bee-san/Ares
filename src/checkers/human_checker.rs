use crate::checkers::checker_result::CheckResult;
use crate::cli_pretty_printing::human_checker_check;
use crate::config::get_config;
use crate::storage::database;
use crate::timer;
use crate::tui::human_checker_bridge::{is_tui_confirmation_active, request_tui_confirmation};
use dashmap::DashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use text_io::read;

/// Set of prompts that have already been shown to avoid duplicate confirmation requests.
static SEEN_PROMPTS: OnceLock<DashSet<String>> = OnceLock::new();

/// Flag indicating whether a human has confirmed a result.
/// Once set to true, future human checker calls will be skipped.
static HUMAN_CONFIRMED: AtomicBool = AtomicBool::new(false);

/// Returns a reference to the set of already-seen prompt keys.
fn get_seen_prompts() -> &'static DashSet<String> {
    SEEN_PROMPTS.get_or_init(DashSet::new)
}

/// Resets human checker state for a fresh run.
///
/// This clears the seen prompts set and resets the confirmation flag,
/// allowing the human checker to prompt again for previously seen candidates.
/// Call this when rerunning Ciphey from the TUI to ensure the human checker
/// behaves as if it's a fresh CLI invocation.
pub fn reset_human_checker_state() {
    HUMAN_CONFIRMED.store(false, Ordering::Release);
    if let Some(prompts) = SEEN_PROMPTS.get() {
        prompts.clear();
    }
}

/// The Human Checker asks humans if the expected plaintext is real plaintext
/// We can use all the automated checkers in the world, but sometimes they get false positives
/// Humans have the last say.
///
/// This function supports two modes:
/// - **TUI mode**: When the TUI confirmation channel is active, uses channel-based
///   communication to display a confirmation modal in the TUI.
/// - **CLI mode**: Falls back to standard stdin input when not in TUI mode.
// compile this if we are not running tests
pub fn human_checker(input: &CheckResult) -> bool {
    // Check if a human has already confirmed a result
    // If so, reject all other candidates - only one result should succeed
    if HUMAN_CONFIRMED.load(Ordering::Acquire) {
        return false;
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
        timer::resume();
        return true; // Return true to allow the search to continue
    }

    // Determine which mode to use for getting user input
    let result = if is_tui_confirmation_active() {
        // TUI mode: use channel-based communication
        match request_tui_confirmation(input) {
            Some(response) => response,
            None => {
                // Channel failed (TUI exited?), treat as rejection
                timer::resume();
                return false;
            }
        }
    } else {
        // CLI mode: use standard stdin input
        human_checker_check(&input.description, &input.text);

        let reply: String = read!("{}\n");
        reply.to_ascii_lowercase().starts_with('y')
    };

    // If the user confirmed, set the atomic boolean to true
    if result {
        HUMAN_CONFIRMED.store(true, Ordering::Release);
    }
    timer::resume();

    if !result {
        // Pass None for encoded_text and decoder_path for now
        // These can be populated later when context threading is implemented
        let fd_result = database::insert_human_rejection(&input.text, input, None, None);
        match fd_result {
            Ok(_) => (),
            Err(e) => {
                log::warn!(
                    "Failed to write human checker rejection due to error: {}",
                    e
                );
            }
        }
        return false;
    }
    true
}
