use std::{
    sync::atomic::AtomicBool,
    thread::{self, sleep},
    time::Duration,
};

use crossbeam::channel::{bounded, Receiver, Sender};

static PAUSED: AtomicBool = AtomicBool::new(false);

pub fn start(duration: u32) -> Receiver<()> {
    let (sender, recv) = bounded(1);
    thread::spawn(move || hacky_timer(sender, duration));
    recv
}

fn hacky_timer(sender: Sender<()>, duration: u32) {
    let mut time_spent = 0;
    loop {
        if time_spent >= duration {
            sender.send(()).expect("Timer should send succesfully");
            break;
        }
        sleep(Duration::from_secs(1));

        if !PAUSED.load(std::sync::atomic::Ordering::Relaxed) {
            time_spent += 1;
        }
    }
}

pub fn pause() {
    // *PAUSED.get_mut() = true;
    PAUSED.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub fn resume() {
    // *PAUSED.get_mut() = false;
    PAUSED.store(false, std::sync::atomic::Ordering::Relaxed);
}
