use std::{pin::{Pin, pin}, time::Duration};


#[test]
fn main1() {
    trpl::run(async {
        
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };


        // Box::new之后的类型，仍然是不同的，因为底层的 async 块生成的匿名类型不同，Box::new 只是将它们包装成 Box<T>，但 T 仍然是不同的匿名类型，因此无法放入同一个 Vec 中。
        // let futures: Vec<Box<dyn Future<Output = ()>>> = 
        //     vec![Box::new(tx1_fut), Box::new(tx_fut), Box::new(rx_fut)];


        // Box::pin 之后的类型，仍然是不同的，因为底层的 async 块生成的匿名类型不同，Box::pin 只是将它们包装成 Pin<Box<T>>，但 T 仍然是不同的匿名类型，因此无法放入同一个 Vec 中。
        // 必须显示的将Pin<Box<T>>里面的T转换成统一的trait object类型，才能放入同一个Vec中。
        // 即显示的转换为dyn Future<Output = ()>这个trait
        // 显示转换为同一个trait
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = 
            vec![Box::pin(tx1_fut), Box::pin(tx_fut), Box::pin(rx_fut)];

        trpl::join_all(futures).await;
    });
}






#[test]
fn main2() {
    trpl::run(async {
        
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });


        // let futures: Vec<Box<dyn Future<Output = ()>>> = 
        //     vec![Box::new(tx1_fut), Box::new(tx_fut), Box::new(rx_fut)];

        // 这个地方为什么要加&mut？
        // 二、从底层原理解释：为什么必须加 &mut？
        // Future 特质的核心方法 poll 定义如下（简化版）：
        // rust
        // 运行
        // trait Future {
        //     type Output;
        //     // 重点看第一个参数：self 必须是 Pin<&mut Self>
        //     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        // }
        // 这个参数决定了：
        // 必须可变（&mut）：
        // Future 执行时会改变自身内部状态（比如从 “等待 sleep” 变为 “执行下一步发送消息”），poll 方法需要修改 Future 的状态，因此必须拿到可变引用（不可变引用 &Self 无法修改状态）。
        // 必须 Pin：
        // pin! 宏的作用是把 Future “固定” 在内存中，防止其被移动（异步运行时需要 Future 内存地址稳定），所以 Pin 是前置条件。
        // 简单说：trpl::join_all 要驱动多个 Future 执行，就必须调用它们的 poll 方法，而调用 poll 必须满足 Pin<&mut Future> 的类型要求 —— 这就是 &mut 存在的根本原因。
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = 
            vec![tx1_fut, tx_fut, rx_fut];

        trpl::join_all(futures).await;
    });
}

