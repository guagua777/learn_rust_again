use tokio::time::sleep;
use std::sync::Mutex;
use std::time::Duration;

static DATA: Mutex<u32> = Mutex::new(0);

async fn task1() {
    println!("Task1 try lock");
    let _guard = DATA.lock().unwrap();
    println!("Task1 locked, sleep 5s");
    sleep(Duration::from_secs(5)).await;
    println!("Task1 done");
}

async fn task2() {
    sleep(Duration::from_millis(100)).await;
    println!("Task2 try lock");
    let _guard = DATA.lock().unwrap();
    println!("Task2 locked");
}

#[tokio::main]
// #[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    tokio::join!(task1(), task2());
}