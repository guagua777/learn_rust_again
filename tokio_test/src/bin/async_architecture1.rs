use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::runtime;


// 原因：rt.spawn() 把任务提交到运行时，但任务只有在运行时被驱动时才会执行。这里没有调用 rt.block_on() 或 rt.run()，运行时从未被轮询，所以 sub task 永远不会运行。

// thread::sleep(3s) 只是让主线程睡眠，它不会驱动 tokio 运行时。

// 修复方式：用 block_on 驱动运行时：


// fn main() {
//     let rt = runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     rt.block_on(async {
//         let h = tokio::spawn(async {
//             println!("sub task begin");
//             thread::sleep(Duration::from_secs(1));
//             println!("sub task done");
//         });
//         h.await.unwrap();
//     });

//     println!("main task done");
// }
// 关键点：new_current_thread() 创建的是单线程运行时，它不会在后台自动运行，必须通过 block_on 显式驱动。如果用 new_multi_thread()，运行时会有后台线程，rt.spawn() 后 thread::sleep 等待足够长时间任务才可能执行——但这也是不可靠的做法。

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.spawn(async {
        println!("sub task begin");
        thread::sleep(Duration::from_secs(1));
        println!("sub task done");
        return;
    });

    thread::sleep(Duration::from_secs(3));
    println!("main task done");
}

