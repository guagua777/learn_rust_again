use std::{thread, time::Duration};


async fn hello(task: u64, time: u64) {
    println!("hello tokio task: {}, running on {:?}", task, thread::current().id());
    thread::sleep(Duration::from_millis(time));
    println!("hello tokio task: {}, done", task);
}


#[tokio::main]
async fn main() {

    let _ = tokio::join!(

        // 生成5个并行的任务，跟调度没有关系
        tokio::spawn(hello(1, 200)),
        tokio::spawn(hello(2, 200)),
        tokio::spawn(hello(3, 200)),
        tokio::spawn(hello(4, 200)),
        tokio::spawn(hello(5, 200)),
    );

}