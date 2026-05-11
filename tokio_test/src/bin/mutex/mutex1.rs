use std::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

async fn incr() {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
}

#[tokio::main]
async fn main() {
    tokio::join!(incr(), incr(), incr());
    println!("COUNTER = {}", *COUNTER.lock().unwrap());
}