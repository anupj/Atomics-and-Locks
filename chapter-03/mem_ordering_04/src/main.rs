use std::{
    sync::atomic::{
        AtomicBool, AtomicU64,
        Ordering::*,
        Ordering::{Acquire, Release},
    },
    thread,
    time::Duration,
};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release);
        // Everything from before this 👆 store ..
    });
    thread::spawn(|| {
        DATA.store(345, Relaxed);
    });
    // .. is visible after this loads `true`
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(1));
        println!("waiting...");
        println!("DATA is {}", DATA.load(Relaxed));
    }
    println!("{}", DATA.load(Relaxed));
}
