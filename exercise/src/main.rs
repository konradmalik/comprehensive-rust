use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread count: {}", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("main count: {}", i);
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap();

    let s = "yo! scoped thread!";

    thread::scope(|scope| {
        scope.spawn(|| {
            for i in 1..10 {
                println!("{}: {}", i, s);
            }
        });
    });
}
