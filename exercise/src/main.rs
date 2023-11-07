use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is trying to eat", &self.name);
        let _left_guard = self.left_fork.lock().unwrap();
        let _right_guard = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create forks
    let mut forks = Vec::with_capacity(5);
    for _ in 0..forks.capacity() {
        forks.push(Arc::new(Mutex::new(Fork)));
    }

    // Create philosophers
    let mut philosophers = Vec::with_capacity(5);
    let (tx, rx) = mpsc::channel();
    for i in 0..philosophers.capacity() {
        let mut p = Philosopher {
            name: PHILOSOPHERS[i].to_owned(),
            left_fork: Arc::clone(&forks[i]),
            right_fork: Arc::clone(&forks[(i + 1) % forks.capacity()]),
            thoughts: tx.clone(),
        };

        // To avoid a deadlock, we have to break the symmetry
        // somewhere. This will swap the forks without deinitializing
        // either of them.
        if i == forks.len() - 1 {
            std::mem::swap(&mut p.left_fork, &mut p.right_fork);
        }

        philosophers.push(p);
    }
    drop(tx);

    // Make each of them think and eat 100 times
    thread::scope(|scope| {
        for philosopher in philosophers.into_iter() {
            scope.spawn(move || {
                for _ in 0..100 {
                    philosopher.think();
                    philosopher.eat();
                }
            });
        }
    });

    // Output their thoughts
    while let Ok(thought) = rx.recv() {
        println!("{}", thought);
    }
}
