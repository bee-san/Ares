//! Bridge between the human checker and TUI for confirmation requests.
//!
//! When running in TUI mode, the human checker cannot use stdin directly
//! because the terminal is in raw mode and using the alternate screen.
//! This module provides a channel-based mechanism for the human checker
//! to request confirmation from the TUI and receive the user's response.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, OnceLock};

use super::app::HumanConfirmationRequest;
use crate::checkers::checker_result::CheckResult;

/// A confirmation request sent from the human checker to the TUI.
///
/// This struct bundles the request details with a response channel,
/// allowing the TUI to send back the user's decision.
pub struct TuiConfirmationRequest {
    /// The request details containing the candidate text and checker info.
    pub request: HumanConfirmationRequest,
    /// Channel to send the user's response (true = accept, false = reject) back.
    pub response_tx: Sender<bool>,
}

/// Global sender for TUI confirmation requests.
///
/// This is wrapped in a Mutex so it can be replaced when rerunning Ciphey.
/// The human checker uses this to send confirmation requests to the TUI.
static TUI_CONFIRMATION_TX: OnceLock<Mutex<Option<Sender<TuiConfirmationRequest>>>> =
    OnceLock::new();

/// Global receiver for TUI confirmation requests, wrapped in a Mutex for thread-safe access.
///
/// The receiver is wrapped in an Option inside a Mutex so it can be taken
/// by the TUI event loop and replaced when rerunning.
static TUI_CONFIRMATION_RX: OnceLock<Mutex<Option<Receiver<TuiConfirmationRequest>>>> =
    OnceLock::new();

/// Initialize the TUI confirmation channel.
///
/// This function creates the channel used for communication between the human checker
/// and the TUI. It must be called before starting the TUI and before any human checker
/// calls that need to use TUI mode.
///
/// # Returns
///
/// `true` if initialization succeeded, `false` if already initialized.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::init_tui_confirmation_channel;
///
/// // Initialize before starting TUI
/// if init_tui_confirmation_channel() {
///     println!("Channel initialized successfully");
/// } else {
///     println!("Channel was already initialized");
/// }
/// ```
pub fn init_tui_confirmation_channel() -> bool {
    let (tx, rx) = mpsc::channel();

    // Try to set both sender and receiver - only succeeds on first call
    // If already initialized, return false (caller should use reinit for re-initialization)
    let tx_set = TUI_CONFIRMATION_TX.set(Mutex::new(Some(tx))).is_ok();
    let rx_set = TUI_CONFIRMATION_RX.set(Mutex::new(Some(rx))).is_ok();

    // Return true only if both were newly set (first initialization)
    tx_set && rx_set
}

/// Reinitialize the TUI confirmation channel for a new decode run.
///
/// This function creates a fresh channel, replacing any existing sender/receiver.
/// Call this when rerunning Ciphey to ensure the human checker can communicate
/// with the TUI event loop properly.
///
/// # Returns
///
/// The new receiver if successful, `None` if the channel system wasn't initialized.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::reinit_tui_confirmation_channel;
///
/// // When rerunning Ciphey from a selected step:
/// if let Some(new_receiver) = reinit_tui_confirmation_channel() {
///     // Use new_receiver in the event loop
/// }
/// ```
pub fn reinit_tui_confirmation_channel() -> Option<Receiver<TuiConfirmationRequest>> {
    // Create a fresh channel
    let (tx, rx) = mpsc::channel();

    // Replace the sender
    let tx_mutex = TUI_CONFIRMATION_TX.get()?;
    if let Ok(mut guard) = tx_mutex.lock() {
        *guard = Some(tx);
    }

    // Return the new receiver (don't store it - caller will use it directly)
    Some(rx)
}

/// Get the receiver for TUI confirmation requests.
///
/// This function takes ownership of the receiver, meaning it can only be called once
/// successfully. Subsequent calls will return `None`. This ensures only one consumer
/// (the TUI event loop) handles confirmation requests.
///
/// # Returns
///
/// `Some(Receiver<TuiConfirmationRequest>)` on first call after initialization,
/// `None` if not initialized or already taken.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::{init_tui_confirmation_channel, take_confirmation_receiver};
///
/// init_tui_confirmation_channel();
///
/// // First call succeeds
/// let receiver = take_confirmation_receiver().expect("Should get receiver");
///
/// // Second call returns None
/// assert!(take_confirmation_receiver().is_none());
/// ```
pub fn take_confirmation_receiver() -> Option<Receiver<TuiConfirmationRequest>> {
    TUI_CONFIRMATION_RX.get()?.lock().ok()?.take()
}

/// Request confirmation from the TUI.
///
/// This function sends a confirmation request to the TUI and blocks until the user
/// responds. It is designed to be called from the human checker when running in
/// TUI mode.
///
/// # Arguments
///
/// * `check_result` - The check result containing the candidate plaintext and checker info.
///
/// # Returns
///
/// * `Some(true)` - The user accepted the plaintext as valid.
/// * `Some(false)` - The user rejected the plaintext.
/// * `None` - TUI mode is not active (channel not initialized), caller should fall back
///   to standard input handling.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::request_tui_confirmation;
/// use ciphey::checkers::checker_result::CheckResult;
///
/// fn check_with_human(result: &CheckResult) -> bool {
///     // Try TUI mode first
///     if let Some(response) = request_tui_confirmation(result) {
///         return response;
///     }
///     // Fall back to CLI input...
///     true
/// }
/// ```
pub fn request_tui_confirmation(check_result: &CheckResult) -> Option<bool> {
    // Get the sender from the Mutex, return None if not initialized or no sender
    let tx_mutex = TUI_CONFIRMATION_TX.get()?;
    let tx = {
        let guard = tx_mutex.lock().ok()?;
        guard.as_ref()?.clone()
    };

    // Create a one-shot channel for the response
    let (response_tx, response_rx) = mpsc::channel();

    // Build the confirmation request
    let request = TuiConfirmationRequest {
        request: HumanConfirmationRequest::from(check_result),
        response_tx,
    };

    // Send the request to the TUI
    // If send fails (TUI has exited), return None
    tx.send(request).ok()?;

    // Block and wait for the response
    // If receive fails (TUI has exited), return None
    response_rx.recv().ok()
}

/// Check if TUI confirmation mode is active.
///
/// This function checks whether the TUI confirmation channel has been initialized,
/// indicating that the application is running in TUI mode and the human checker
/// should use channel-based communication instead of direct stdin.
///
/// # Returns
///
/// `true` if the channel has been initialized (TUI mode is active),
/// `false` otherwise.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::is_tui_confirmation_active;
///
/// if is_tui_confirmation_active() {
///     // Use request_tui_confirmation()
/// } else {
///     // Use standard CLI input
/// }
/// ```
pub fn is_tui_confirmation_active() -> bool {
    TUI_CONFIRMATION_TX
        .get()
        .and_then(|m| m.lock().ok())
        .map(|guard| guard.is_some())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing channel initialization is difficult because OnceLock persists
    // across tests in the same process. True unit tests would require process
    // isolation. The channel logic is tested indirectly via integration tests.

    /// Create a test CheckResult for use in tests.
    fn make_test_check_result() -> CheckResult {
        CheckResult {
            is_identified: true,
            text: "test plaintext".to_string(),
            description: "Test description".to_string(),
            checker_name: "TestChecker",
            checker_description: "A test checker",
            link: "https://example.com",
        }
    }

    #[test]
    fn test_human_confirmation_request_from_check_result() {
        let check_result = make_test_check_result();
        let request = HumanConfirmationRequest::from(&check_result);

        assert_eq!(request.text, "test plaintext");
        assert_eq!(request.description, "Test description");
        assert_eq!(request.checker_name, "TestChecker");
    }
}
