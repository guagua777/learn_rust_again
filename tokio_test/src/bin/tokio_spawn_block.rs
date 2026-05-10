use std::{thread, time::Duration};
use tokio::task::spawn_blocking;


async fn delay(task: u32, time: u64) {
    println!("task {task} start");

    let result = spawn_blocking(move || {
        thread::sleep(Duration::from_millis(time));
        time
    }).await;

    println!("task {task} result: {result:#?}");

    println!("task {task} done");
}



#[tokio::main]
async fn main() {

    let _ = tokio::join!(
        delay(1, 200),
        delay(2, 200),
        delay(3, 200),
    );

    println!("all done");
    
}