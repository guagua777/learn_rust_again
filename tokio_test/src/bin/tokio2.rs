
use tokio::{join, task::JoinSet};


async fn hello() {
    println!("hello tokio");
}

async fn run() {
    for i in 0..10 {
        println!("i = {}", i);
        tokio::task::yield_now().await;
    }
}

// 关键是找出调度的单位
#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        // 调度的分界线，或者说是调度的单位
        // 一个调度单位
        tokio::spawn(hello()),
        // 另一个调度单位
        tokio::spawn(run()),
        // 第三个调度单位
        tokio::spawn(run()),
    );

    println!("done");
}

// 调度器里面：
// 一个continuation为一个调度单位
// 可以多个contintuation为一个调度单位