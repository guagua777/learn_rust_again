use std::time::Duration;

use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};




async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => println!("从 mpsc channel 收到消息 {n}"),
            Ok(n) = broadcast_rx.recv() => println!("从 broadcast channel 收到消息 {n}"),
        }
    }
}


#[tokio::main]
async fn main() {

    // 定义两个channel
    let (tx, rx) = mpsc::channel::<u32>(1);
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

    // 开启一个异步任务
    tokio::spawn(receiver(rx, broadcast_rx));

    // 发送消息
    for c in 0..10 {
        if c % 2 == 0 {
            tx.send(c).await.unwrap();
        } else {
            broadcast_tx.send(c).unwrap();
        }
        sleep(Duration::from_secs(1)).await;
    }
}

