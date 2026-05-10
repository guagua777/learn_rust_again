use std::{thread, time::Duration};
use tokio::task::spawn_blocking;


async fn delay(task: u32, time: u64) {
    println!("task {task} start");

    spawn_blocking(move || {
        thread::sleep(Duration::from_millis(time));
        println!("blocking ... ");
        
        time
    });

    println!("task {task} done");
}


#[tokio::main(flavor = "current_thread")]
// #[tokio::main]
async fn main() {

    let _ = tokio::join!(
        delay(1, 200),
        delay(2, 200),
        delay(3, 200),
    );

    println!("all done");
    
}