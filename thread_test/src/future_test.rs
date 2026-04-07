use std::{thread::yield_now, time::Duration};

use trpl::Html;





#[test]
fn main1() {

    trpl::run(
        // 里面为async块
        async {
        println!("hello async world");
    })
}


#[test]
fn main2() {

    trpl::run(
        // 里面为async块
        async {
            // 此时该async块，没有await，所以不会执行。
            async {
                println!("hello async world");
            };
    })
}


#[test]
fn main3() {

    trpl::run(
        // 里面为async块
        async {            
            async {
                println!("hello async world");
            }.await;

          
    })
}




#[test]
fn main4() {

    trpl::run(
        // 里面为async块
        async {   

            // 两个async块，都加await，只能是顺序执行。
            // 如何让两个async块，一起执行？
            // 使用trpl::join
            async {
                println!("hello async111 world");
            }.await;

            async {
                println!("hello async222 world");
            }.await;
    })
}

// 代码分析：为什么 "发送完成。" 会在 4 个 2 秒后才打印
// 问题分析
// 在您的代码中，"发送完成。" 语句确实会在 4 个 2 秒后才打印，这是由 Rust 异步代码的执行机制决定的。让我详细解释一下：

// 代码执行流程
// 创建通道：let (tx, mut rx) = trpl::channel();
// 准备数据：创建包含 4 个字符串的向量
// 循环发送数据：
// 发送值到通道：tx.send(val).unwrap();
// 打印 "依次发送..."
// 关键操作：trpl::sleep(Duration::from_secs(2)).await;
// 打印完成信息：println!("发送完成。");
// 接收数据：循环接收并打印通道中的值
// 为什么会等待 4 个 2 秒
// 核心原因：await 关键字会暂停当前异步任务的执行，直到等待的操作完成。

// 具体来说：

// 当执行到 trpl::sleep(Duration::from_secs(2)).await; 时，当前任务会被暂停
// 2 秒后，任务会从暂停的地方继续执行
// 循环会重复这个过程 4 次，每次都暂停 2 秒
// 只有当 4 次循环都执行完毕后，才会执行 println!("发送完成。");
// 异步执行机制
// 在 Rust 的异步模型中：

// async 块创建一个 Future
// await 会让出执行权，允许其他任务运行
// 但在这个例子中，所有操作都在同一个 async 块中顺序执行
// 没有使用并发机制（如 trpl::spawn_task 或 trpl::join）
// 对比：并发执行的方式
// 如果您希望在发送的同时接收数据，可以参考 async_concurrency.rs 中的 main5 函数，使用 trpl::join 来并发执行发送和接收操作：


// rust
// let tx_fut = async {
//     // 发送逻辑
// };

// let rx_fut = async {
//     // 接收逻辑
// };

// trpl::join(tx_fut, rx_fut).await;
// 这样，发送和接收可以同时进行，而不是等待所有发送完成后再接收。

// 总结
// "发送完成。" 会在 4 个 2 秒后才打印，是因为：

// 循环中的每个 trpl::sleep(Duration::from_secs(2)).await 都会暂停执行 2 秒
// 这些暂停是顺序执行的，总共需要 4 * 2 = 8 秒
// 只有当所有发送操作和等待时间都完成后，才会执行 "发送完成。" 的打印语句
// 这是异步代码的正常行为，await 确保了操作的顺序执行。

#[test]
fn main5() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

       
        for val in vals {
            tx.send(val).unwrap();
            println!("依次发送...");
            // trpl::sleep(Duration::from_millis(500)).await;
            trpl::sleep(Duration::from_secs(2)).await;
        }
        // 这里为什么不是，立即就打印，而是等待了4个2秒后，才打印？
        println!("发送完成。");

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }

    });
}


// async块内部是顺序执行的

#[test]
fn main6() {
    trpl::run(
        // 这是一个continue
        async {
            println!("continue1 inner running1-1...");
            println!("continue1 inner running1-2...");
            println!("continue1 inner running1-3...");
            // 后面又是一个continue
            trpl::sleep(Duration::from_secs(2)).await;
            println!("continue2 inner running2-1...");
            println!("continue2 inner running2-2...");
            println!("continue2 inner running2-3...");
    });

    // 如果只有continue，而没有调度，continue内部就会顺序执行
    // 只有添加调度，continue才会切换执行，所以需要使用trpl::spawn_task 或 trpl::join等调度器

    // async块内部是顺序执行的，因为这是一个continue

}



#[test]
fn main7() {
    trpl::run(
        // 这是一个continue
        async {
            let f1 = async {
                println!("continue1 inner running1-1...");
                println!("continue1 inner running1-2...");
                println!("continue1 inner running1-3...");
                // 后面又是一个continue
                trpl::sleep(Duration::from_secs(2)).await;
                println!("continue2 inner running2-1...");
                println!("continue2 inner running2-2...");
                println!("continue2 inner running2-3...");
            };


            let f2 = async {
                println!("continue1 ------ inner running1-1...");
                println!("continue1 ------ inner running1-2...");
                println!("continue1 ------ inner running1-3...");
                // 后面又是一个continue
                trpl::sleep(Duration::from_secs(2)).await;
                println!("continue2 ------ inner running2-1...");
                println!("continue2 ------ inner running2-2...");
                println!("continue2 ------ inner running2-3...");
            };

            // 添加调度器后，两个continue就可以切换运行了
            trpl::join(f1, f2).await;


    });

    // 如果只有continue，而没有调度，continue内部就会顺序执行
    // 只有添加调度，continue才会切换执行，所以需要使用trpl::spawn_task 或 trpl::join等调度器

    // async块内部是顺序执行的，因为这是一个continue

}






async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}


fn main10() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(
        // 里面为async块
        async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

