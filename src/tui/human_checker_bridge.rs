//! Bridge between the human checker and TUI for confirmation requests.
//!
//! When running in TUI mode, the human checker cannot use stdin directly
//! because the terminal is in raw mode and using the alternate screen.
//! This module provides a channel-based mechanism for the human checker
//! to request confirmation from the TUI and receive the user's response.
//!
//! ## Design
//!
//! Global state uses `Mutex<Option<T>>` (no `OnceLock`) so that
//! [`init_tui_confirmation_channel`] can be called multiple times within the
//! same process  each call simply replaces the inner value. This removes the
//! previous dual-code-path problem where the first call used `OnceLock::set()`
//! and subsequent calls had to go through a different `reinit` path.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;

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
/// The human checker clones the inner `Sender` to dispatch requests to the TUI.
/// Wrapped in `Mutex<Option<&>>` so it can be replaced on every
/// [`init_tui_confirmation_channel`] call.
static TUI_CONFIRMATION_TX: Mutex<Option<Sender<TuiConfirmationRequest>>> = Mutex::new(None);

/// Global receiver for TUI confirmation requests.
///
/// The receiver is wrapped in `Mutex<Option<&>>` so it can be taken
/// by the TUI event loop via [`take_confirmation_receiver`] and replaced
/// when re-initialising.
static TUI_CONFIRMATION_RX: Mutex<Option<Receiver<TuiConfirmationRequest>>> = Mutex::new(None);

/// Initialize (or re-initialize) the TUI confirmation channel.
///
/// Creates a fresh `mpsc` channel and stores the sender and receiver in the
/// global statics. This function is idempotent  calling it multiple times
/// simply replaces the previous channel, which is exactly what we want when
/// rerunning a decode from the TUI.
///
/// # Returns
///
/// Always returns `true`. The return value is kept for backward compatibility
/// but callers no longer need to distinguish first-init from re-init.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::init_tui_confirmation_channel;
///
/// // Works on first call&
/// init_tui_confirmation_channel();
///
/// // &and on subsequent calls (replaces the channel).
/// init_tui_confirmation_channel();
/// ```
pub fn init_tui_confirmation_channel() -> bool {
    let (tx, rx) = mpsc::channel();

    if let Ok(mut guard) = TUI_CONFIRMATION_TX.lock() {
        *guard = Some(tx);
    }
    if let Ok(mut guard) = TUI_CONFIRMATION_RX.lock() {
        *guard = Some(rx);
    }

    true
}

/// Re-initialize the TUI confirmation channel and return the new receiver.
///
/// This is a convenience wrapper around [`init_tui_confirmation_channel`]
/// that also takes the freshly-created receiver out of the global static
/// so the caller can use it directly in the event loop.
///
/// # Returns
///
/// `Some(Receiver)` on success, `None` if the mutex is poisoned.
///
/// # Example
///
/// ```ignore
/// use ciphey::tui::human_checker_bridge::reinit_tui_confirmation_channel;
///
/// if let Some(new_receiver) = reinit_tui_confirmation_channel() {
///     // Use new_receiver in the event loop
/// }
/// ```
pub fn reinit_tui_confirmation_channel() -> Option<Receiver<TuiConfirmationRequest>> {
    init_tui_confirmation_channel();
    take_confirmation_receiver()
}

/// Take ownership of the confirmation receiver.
///
/// Returns `Some(Receiver)` if a receiver is available, `None` otherwise.
/// After this call the global receiver slot is empty until the next
/// [`init_tui_confirmation_channel`] call.
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
/// // Second call returns None (receiver already taken)
/// assert!(take_confirmation_receiver().is_none());
/// ```
pub fn take_confirmation_receiver() -> Option<Receiver<TuiConfirmationRequest>> {
    TUI_CONFIRMATION_RX.lock().ok()?.take()
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
    // Clone the sender out of the Mutex so we don't hold the lock while blocking.
    let tx = {
        let guard = TUI_CONFIRMATION_TX.lock().ok()?;
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
/// Returns `true` when a sender is present in the global slot, meaning
/// the TUI has been initialised and the human checker should use
/// channel-based communication instead of direct stdin.
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
        .lock()
        .ok()
        .map(|guard| guard.is_some())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_init_channel_is_idempotent() {
        // First init
        assert!(init_tui_confirmation_channel());
        assert!(is_tui_confirmation_active());

        // Second init replaces the channel  still succeeds
        assert!(init_tui_confirmation_channel());
        assert!(is_tui_confirmation_active());
    }

    #[test]
    fn test_take_receiver_returns_none_after_take() {
        init_tui_confirmation_channel();

        // First take succeeds
        let rx = take_confirmation_receiver();
        assert!(rx.is_some());

        // Second take returns None (receiver was taken)
        let rx2 = take_confirmation_receiver();
        assert!(rx2.is_none());
    }

    #[test]
    fn test_reinit_provides_fresh_receiver() {
        init_tui_confirmation_channel();
        // Take the first receiver
        let _ = take_confirmation_receiver();

        // Reinit creates a new channel and returns its receiver
        let rx = reinit_tui_confirmation_channel();
        assert!(rx.is_some());
    }

    #[test]
    fn test_request_returns_none_when_not_initialised() {
        // Clear any existing sender
        if let Ok(mut guard) = TUI_CONFIRMATION_TX.lock() {
            *guard = None;
        }

        let check_result = make_test_check_result();
        assert!(request_tui_confirmation(&check_result).is_none());
    }
}
