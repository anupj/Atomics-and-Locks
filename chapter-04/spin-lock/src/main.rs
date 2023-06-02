use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::*;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { locked: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    // The lifetime for input(`&self`) and
    // output(`&mut T`) is elided here and
    // is assumed to be identical.
    // This means that the returned reference
    // is valid as long as the lock itself exists.
    pub fn lock<'a>(&'a self) -> &'a mut T {
        // could've used a `compare_exchange_weak`
        // here instead, its a matter of taste
        while self.locked.swap(true, Acquire) {
            // This tells the processor
            // that we are spinning while
            // waiting for something to change.
            // It doesn't cause the OS to be called
            // to  put your thread to sleep in favour
            // of another thread.
            std::hint::spin_loop();
        }
        // Safety: If I reach this point means that
        // I have the lock this is safe to do
        unsafe { &mut *self.value.get() }
        // 👆 here we get a `*mut T` (pointer to the
        // wrapped value and we are casting to an exclusive
        // reference)
    }

    // Safety: The `&mut T` from lock() must be gone!
    // (And no cheating by keeping reference to fields
    // of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    println!("Hello, world!");
}
