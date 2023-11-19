use tokio::sync::oneshot::{self, Receiver};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum Animal {
    Cat { name: String },
    Dog { name: String },
}

async fn first_animal_to_finish_race(
    cat_rcv: Receiver<String>,
    dog_rcv: Receiver<String>,
    race_rcv: Receiver<()>,
) -> Option<Animal> {
    tokio::select! {
        cat_name = cat_rcv => Some(Animal::Cat { name:cat_name.expect("cannot receive cat name")}),
        dog_name = dog_rcv => Some(Animal::Dog { name:dog_name.expect("cannot receive dog name")}),
        _ = race_rcv => None,
    }
}

#[tokio::main]
async fn main() {
    let (cat_sender, cat_receiver) = oneshot::channel();
    let (dog_sender, dog_receiver) = oneshot::channel();
    let (race_sender, race_receiver) = oneshot::channel();
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        cat_sender
            .send(String::from("Felix"))
            .expect("Problem sending cat");
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        dog_sender
            .send(String::from("Rex"))
            .expect("Problem sending dog");
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        race_sender.send(()).expect("Problem sending race finish");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver, race_receiver).await;

    println!("Winner is {winner:?}");
}
