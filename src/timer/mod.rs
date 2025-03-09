use crossbeam::channel::{bounded, Receiver};
use std::sync::atomic::Ordering::Relaxed;
use std::{
    sync::atomic::AtomicBool,
    thread::{self, sleep},
    time::Duration,
};

use crate::cli_pretty_printing::{countdown_until_program_ends, display_top_results};
use crate::config::get_config;
use crate::storage::wait_athena_storage;

/// Indicate whether timer is paused
static PAUSED: AtomicBool = AtomicBool::new(false);

/// Start the timer with duration in seconds
pub fn start(duration: u32) -> Receiver<()> {
    let (sender, recv) = bounded(1);
    thread::spawn(move || {
        let mut time_spent = 0;

        while time_spent < duration {
            if !PAUSED.load(Relaxed) {
                sleep(Duration::from_secs(1));
                time_spent += 1;
                // Some pretty printing support
                countdown_until_program_ends(time_spent, duration);
            }
        }

        // When the timer expires, display all collected plaintext results
        // Only if we're in top_results mode
        let config = get_config();
        log::trace!("Timer expired. top_results mode: {}", config.top_results);

        if config.top_results {
            log::info!("Displaying all collected plaintext results");
            filter_and_display_results();
        } else {
            log::info!("Not in top_results mode, skipping display_wait_athena_results()");
        }

        sender.send(()).expect("Timer should send succesfully");
    });

    recv
}

/// Filter and display all plaintext results collected by WaitAthena
fn filter_and_display_results() {
    let results = wait_athena_storage::get_plaintext_results();

    log::trace!(
        "Retrieved {} results from wait_athena_storage",
        results.len()
    );

    // Use the cli_pretty_printing function to display the results
    display_top_results(&results);
}

/// Pause timer
pub fn pause() {
    PAUSED.store(true, Relaxed);
}

/// Resume timer
pub fn resume() {
    PAUSED.store(false, Relaxed);
}
