use std::sync::atomic::Ordering::*;
use std::sync::atomic::*;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn f() {
    let x = X.load(Relaxed);
    println!(" the value of x is {x}");
    assert!(x == 1 || x == 2);
}
