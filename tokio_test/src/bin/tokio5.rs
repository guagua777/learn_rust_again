use std::{thread, time::Duration};


async fn hello(task: u64, time: u64) {
    println!("hello tokio task: {}, running on {:?}", task, thread::current().id());
    // thread::sleep(Duration::from_millis(time));
    // 返回调度器
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("hello tokio task: {}, done", task);
}


#[tokio::main]
async fn main() {

    // let v = hello(1, 200);

    // 为什么结果是顺序执行的
    tokio::join!(
        // 调度单位
        // 共有5个调度单位
        hello(1, 200),
        hello(2, 200),
        hello(3, 200),
        hello(4, 200),
        hello(5, 200),
    );

}