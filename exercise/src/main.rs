use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        for i in 1..10 {
            println!("sending {}", i);
            tx.send(i).unwrap();
            println!("sent {}", i);
        }
    });

    while let Ok(v) = rx.recv() {
        println!("received {}", v);
    }

    println!("done");
}
