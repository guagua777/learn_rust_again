use tokio::sync::broadcast::{Sender, Receiver};

#[tokio::main]
async fn main() {
    let (tx, mut rx) =
        tokio::sync::broadcast::channel::<String>(16);

    for n in 0..20 {
        let mut messages: Receiver<String> = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = messages.recv().await {
                println!("{n}: {msg}");
            }
        });
    }

    tx.send("Hello channel".to_string()).unwrap();

    while let Ok(msg) = rx.recv().await {
        println!("Main: {msg}");
    }
}