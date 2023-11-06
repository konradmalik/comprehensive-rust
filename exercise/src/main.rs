use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

    let v2 = Arc::clone(&v);
    let handle = thread::spawn(move || {
        let mut guard = v2.lock().unwrap();
        guard.push(10);
    });

    {
        let mut guard = v.lock().unwrap();
        guard.push(1000);
    }

    handle.join().unwrap();
    println!("v: {v:?}");
}
