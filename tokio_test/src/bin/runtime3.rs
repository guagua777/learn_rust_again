use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

// #[tokio::main]
// async  fn main() {
//     // 跟线程池一样，立刻返回
//     let h = tokio::task::spawn_blocking(|| {
//         std::thread::sleep(Duration::from_secs(10)); // 这个睡眠不会被取消
//         println!("sleep finish");
//     });

//     println!("spawn_blocking finish");

//     // drop(h);
//     // 线程依旧会睡满10秒，runtime 后台还在跑这个任务
// }

//共用外面的运行时，
// #[tokio::main]
// async fn main() {
//     // 跟线程池一样，立刻返回
//     let h = tokio::spawn(async move {
//         std::thread::sleep(Duration::from_secs(2));
//         // tokio::time::sleep(Duration::from_secs(2)).await;
//         println!("sleep finish");
//     });

//     println!("spawn finish");
// }

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // 已经提交的同步任务无法取消
    rt.spawn(async move {
        // tokio::time::sleep(Duration::from_secs(2)).await;
        // println!("sleep first finish");
        std::thread::sleep(Duration::from_secs(1));
        println!("sleep second finish");
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("sleep third finish");
        std::thread::sleep(Duration::from_secs(2));
        println!("sleep finish");
    });

    println!("spawn finish");
    // rt在这里隐式drop，runtime销毁！
}
