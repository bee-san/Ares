use crossbeam::channel::{bounded, Receiver};
use std::sync::atomic::Ordering::Relaxed;
use std::{
    sync::atomic::AtomicBool,
    thread::{self, sleep},
    time::Duration,
};

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
                if time_spent % 5 == 0 && time_spent != 0 {
                    println!(
                        "{} seconds have passed. {} remaining",
                        time_spent,
                        duration - time_spent
                    );
                }
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
