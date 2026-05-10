use std::{thread, time::Duration};


async fn hello(task: u64, time: u64) {
    println!("hello tokio task: {}, running on {:?}", task, thread::current().id());
    thread::sleep(Duration::from_millis(time));
    println!("hello tokio task: {}, done", task);
}


#[tokio::main]
async fn main() {

    // let v = hello(1, 200);

    // 为什么结果是顺序执行的
    tokio::join!(
        // 调度单位
        // 共有5个调度单位
        // 但是每个调度单位里面并没有continuation，所以调度单位之间不会调度，只会顺序运行
        hello(1, 200),
        hello(2, 200),
        hello(3, 200),
        hello(4, 200),
        hello(5, 200),
    );

}