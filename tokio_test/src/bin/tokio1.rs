// async fn hello() {
//     println!("hello tokio");
// }

use tokio::{join, task::JoinSet};


async fn hello() {
    for i in 0..10000 {
        let _ = i;
    }
    println!("hello tokio");
}

async fn run() {
    for i in 0..10 {
        println!("i = {}", i);
    }
}

async fn add(a: i32, b: i32) -> i32{
    println!("a = {}, b = {}", a, b);
    a + b   
}

#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(run()),
        tokio::spawn(run()),
    );

    println!("done");
}


// #[tokio::main]
async fn main1() {
    // 生成一个新的async task
    // tokio::spawn(run());
    // hello().await;


    // let res = join!(add(1, 2), add(3, 4));
    // println!("res = {:?}", res);


    let mut join_set = JoinSet::new();
    for i in 0..10 {
        join_set.spawn(add(i, 2));
    }

    while let Some(res) = join_set.join_next().await {
        println!("res = {:?}", res);
    }

   }
