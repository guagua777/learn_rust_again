use std::thread;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(2);

    // 普通同步线程
    thread::spawn(move || {
        for i in 1..=3 {
            println!("同步线程发送: {}", i);
            // 同步阻塞发送，给异步通道用
            tx.blocking_send(i).unwrap();
        }
    });

    while let Some(v) = rx.recv().await {
        println!("异步接收: {}", v);
    }
}