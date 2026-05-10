use futures::executor::block_on;
use trpl::join;


// 调度器，以async块为单位进行调度

async fn hi() {
    println!("Hello");
    let hello = hello();
    // 等待，直到future运行完成
    hello.await;
    println!("after hello world")
}


async fn hello() {
    println!("Hello, world!");
    hello_sync();
}


fn hello_sync() {
    println!("sync hello world");
}

async fn multi() {
    join!(hi(), hello());
    let sum = add(1, 2).await;
    println!("sum: {}", sum);

    let (a, b) = join!(add(1, 2), add(3, 2));
    println!("a: {}, b: {}", a, b);
    
}


async fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let hi = hi();
    // trpl::run(hi);
    // block_on(hi);
    block_on(multi());
}
