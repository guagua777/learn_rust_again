// std::sync::mpsc（标准库）
// 同步阻塞通道：用于线程间通信（OS 线程）。
// 调用 send/recv 会阻塞整个线程，不能用于 async 函数。
// tokio::sync::mpsc（Tokio）
// 异步非阻塞通道：用于Tokio 任务间通信（轻量级协程）。
// 调用 .await 时挂起当前任务，释放线程去跑其他任务，不阻塞线程。


// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     // 有界通道，缓冲容量 2
//     let (tx, rx) = mpsc::sync_channel(2);

//     // 生产者线程
//     let handle = thread::spawn(move || {
//         for i in 1..=5 {
//             println!("发送: {}", i);
//             // 缓冲区满了会 **阻塞当前OS线程**
//             tx.send(i).unwrap();
//         }
//     });

//     // 主线程消费者
//     for _ in 1..=5 {
//         // 没数据会 **阻塞主线程**
//         let val = rx.recv().unwrap();
//         println!("接收: {}", val);
//     }

//     handle.join().unwrap();
// }


use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 有界通道，缓冲容量 2
    let (tx, mut rx) = mpsc::channel(2);

    // 生产者异步任务
    tokio::spawn(async move {
        for i in 1..=5 {
            println!("Tokio 发送: {}", i);
            // 缓冲区满了：**挂起当前任务，不阻塞线程**
            tx.send(i).await.unwrap();
        }
    });

    // 消费者
    while let Some(val) = rx.recv().await {
        println!("Tokio 接收: {}", val);
    }
}


