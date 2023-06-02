use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::*;
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..20 {
            s.spawn(f);
        }
    });
    unsafe { println!("DATA is now {:?}", DATA) };
}
