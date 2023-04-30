use std::sync::atomic::Ordering::*;
use std::sync::atomic::*;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    thread::scope(|s| {
        s.spawn(a);
        s.spawn(b);
    });
    // let t1 = thread::spawn(a);
    // let t2 = thread::spawn(b);
    // t1.join().unwrap();
    // t2.join().unwrap();
}

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}
