use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello, from the main thread.");
    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3];

    // let t = thread::spawn(move || {
    //     let len = numbers.len();
    //     let sum = numbers.iter().sum::<usize>();
    //     sum / len
    // });

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    use std::rc::Rc;

    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread is: {id:?}");
}
