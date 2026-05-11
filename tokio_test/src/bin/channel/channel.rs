use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;


// 核心区别：tokio::spawn 要求在 Tokio 运行时上下文中调用，而 handle.spawn 可以在任意线程中调用。
// 这个文件里，handle.spawn 是在 thread::spawn 创建的普通 OS 线程内部调用的（第12行）。普通线程没有 Tokio 运行时上下文，所以直接用 tokio::spawn 会 panic。
// handle 是在主线程（有运行时上下文）里通过 tokio::runtime::Handle::current() 拿到的运行时句柄，把它 move 进普通线程后，就能通过 handle.spawn 把 async task 提交到原来的 Tokio 运行时去执行。
// 简单说：
// tokio::spawn — 必须在 async 上下文 / Tokio 线程内调用
// handle.spawn — 可以跨线程边界，把 task 投递到持有该 handle 的运行时


#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Task>();
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<i32>(20);
    let handle = tokio::runtime::Handle::current();

    // 系统线程，负责cpu密集型计算
    thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            match task {
                Task::Calculate(n) => {
                    let tx_reply = tx_reply.clone();
                    let result = n * n;
                    // 发送计算后的结果
                    //tokio::spawn(future)
                    handle.spawn(async move {
                        tx_reply.send(result).await.unwrap();
                    });

                },
            }
        }
    });


    tokio::spawn(async move {
        while let Some(result) = rx_reply.recv().await {
            println!("Result: {}", result);
        }
    });


    let mut num: i32 = 1;
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        tx.send(Task::Calculate(num)).unwrap();
        num += 1;
    }
}

enum Task {
    Calculate(i32),
}