use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time;

// Can you make your implementation single-threaded?

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .await
            .unwrap();
    }

    async fn eat(&self) {
        // Pick up forks...
        let _ = self.left_fork.lock().await;
        println!("{} picked up left fork", &self.name);
        let _ = self.right_fork.lock().await;
        println!("{} picked up right fork", &self.name);
        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

#[tokio::main]
async fn main() {
    // Create forks
    let forks: Vec<Arc<Mutex<Fork>>> = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect();

    // Create philosophers
    let (sender, mut receiver) = mpsc::channel(10);

    for (i, name) in PHILOSOPHERS.iter().enumerate() {
        let lf = Arc::clone(&forks[i]);
        let rf = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);
        let thoughts = sender.clone();

        let phil = Philosopher {
            name: name.to_string(),
            left_fork: lf,
            right_fork: rf,
            thoughts,
        };

        tokio::spawn(async move {
            // Make them think and eat
            for _ in 0..100 {
                phil.think().await;
                phil.eat().await;
            }
        });
    }
    drop(sender);

    // Output their thoughts
    while let Some(thought) = receiver.recv().await {
        println!("Thoughtful! {}", thought);
    }
}
