use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;
use std::{sync, thread};

fn main() {
    let queue = Mutex::new(VecDeque::new());
    // Here's our conditional variable
    let not_empty = sync::Condvar::new();

    thread::scope(|s| {
        //First thread spawned
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    // if queue is empty i.e.
                    // `pop_front` return `None`
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        // another thread?
        // or is this main thread?
        for i in 0..10 {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
