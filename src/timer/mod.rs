use crossbeam::channel::{bounded, Receiver};
use std::sync::atomic::Ordering::Relaxed;
use std::{
    sync::atomic::AtomicBool,
    thread::{self, sleep},
    time::Duration,
};

use crate::cli_pretty_printing::countdown_until_program_ends;

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
        sender.send(()).expect("Timer should send succesfully");
    });

    recv
}

/// Pause timer
pub fn pause() {
    PAUSED.store(true, Relaxed);
}

/// Resume timer
pub fn resume() {
    PAUSED.store(false, Relaxed);
}
