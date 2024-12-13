use tokio::{spawn, time::{sleep, Duration}};

async fn say_hello() {
    // Wait for a while before printing to make it a more interesting race.
    sleep(Duration::from_millis(200)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(200)).await;
    println!("world!");
}

#[tokio::main]
async fn main() {
    spawn(say_hello());
    spawn(say_world());

    let a =spawn(say_hello());
    let b =spawn(say_world());

    let _ = a.await;
    let _ = b.await;
    println!("Final before sleep in main");
    // Wait for a while to give the tasks time to run.
    sleep(Duration::from_millis(100)).await;
    println!("Final after sleep in main");
}
