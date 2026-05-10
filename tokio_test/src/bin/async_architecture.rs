use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::runtime;


// Arc 有 内部可变性（Interior Mutability）吗？
// 最精准、最直白的回答：
// Arc 本身没有内部可变性！
// Arc 本身没有内部可变性！
// Arc 本身没有内部可变性！
// 1. 先把概念钉死
// Arc<T>：只做一件事 ——多线程共享所有权（原子引用计数）。
// 内部可变性：在不可变引用 &T 下，修改内部数据。
// Arc<T> 继承了 Rust 的可变性规则：
// 如果内部 T 是不可变的 → 你完全不能修改 Arc 里的内容
// Arc 只提供 &T 共享访问，不提供 &mut T 独占访问
// 所以：
// rust
// 运行
// let arc = Arc::new(5);
// arc = 10; // 报错！
// *arc = 10; // 报错！
// 2. 那我们平时说的 Arc<Mutex<T>> 是什么？
// 这是 组合模式：
// Arc：负责多线程共享所有权
// Mutex / RwLock：负责多线程安全的内部可变性
// 它们是合作关系，不是谁包含谁。
// rust
// 运行
// Arc<Mutex<T>>
// = 共享所有权 + 线程安全内部可变性
// 对应单线程版本：
// rust
// 运行
// Rc<RefCell<T>>
// = 共享所有权 + 单线程内部可变性
// 3. 一张表彻底搞懂
// 表格
// 组合	共享所有权	内部可变性	线程安全
// Arc<T>	✅ 是	❌ 无	✅ 是
// Arc<Mutex<T>>	✅ 是	✅ 有	✅ 是
// Rc<T>	✅ 是	❌ 无	❌ 否
// Rc<RefCell<T>>	✅ 是	✅ 有	❌ 否
// 4. 超级直白总结
// Arc 只是个共享指针，不能改数据。
// Mutex / RwLock 才是提供内部可变性的工具。
// Arc<Mutex<T>> = 多线程共享 + 可修改
// 最终一句话
// Arc 没有内部可变性，必须搭配 Mutex / RwLock 才能在多线程下修改数据。



fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("async block start");
        println_current_time();

        let h1 = tokio::spawn(async {
            println!("sub task begin");
            println_current_time();
            thread::sleep(Duration::from_secs(3));
            println!("sub task done");
            println_current_time();
        });

        let h2_1: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
        let h2_1_inner = h2_1.clone();
        // 不需要加mut，
        // let mut h2_1_inner = h2_1.clone();

        // let h2_1: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
        // let h2

        let h2 = tokio::spawn(async move {
            println!("sub task 2----- begin");
            println_current_time();

            let handle = tokio::spawn(async {
                println!("sub task 2 sub task 1 ----- begin");
                println_current_time();
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("sub task 2 sub task 1 ----- done");
                println_current_time();
            });
            *(h2_1_inner.lock().unwrap()) = Some(handle);

            tokio::time::sleep(Duration::from_secs(10)).await;
            println!("sub task 2----- done");
            println_current_time();
        });

        println!("async block sleep begin");
        println_current_time();
        tokio::time::sleep(Duration::from_secs(6)).await;
        println!("async block sleep end");
        println_current_time();

        let h2_1_handle = h2_1.lock().unwrap().take();
        if let Some(h) = h2_1_handle {
            tokio::join!(h1, h2, h);
        } else {
            tokio::join!(h1, h2);
        }
        println!("async block done");
        println_current_time();
    });

    println!("main task done");
    println_current_time();
}

// continuation之间，可以任意组合
// continuation即异步任务，一个调度单位，一个async块
fn main4() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(
        // 一个异步块，一个调度单位
        async {
            println!("async block start");
            println_current_time();

            // 生成一个调度单位
            tokio::spawn(
                // 阻塞任务，不会让出调度任务
                async {
                    println!("sub task begin");
                    println_current_time();
                    thread::sleep(Duration::from_secs(3));
                    println!("sub task done");
                    println_current_time();
                    return;
                },
            );

            tokio::spawn(async {
                println!("sub task 2----- begin");
                println_current_time();

                tokio::spawn(async {
                    println!("sub task 2 sub task 1 ----- begin");
                    println_current_time();

                    tokio::time::sleep(Duration::from_secs(1)).await;
                    println!("sub task 2 sub task 1 ----- done");
                    println_current_time();
                });

                // 让出调度任务
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("sub task 2----- done");
                println_current_time();
            });

            println!("async block sleep");
            println_current_time();
            // continuation， 让出调度任务
            tokio::time::sleep(Duration::from_secs(6)).await;
            println!("async block done");
            println_current_time();
        },
    );

    println!("main task done");
    println_current_time();
}

// 运行结果
// async block start
// async block sleep
// sub task begin
// sub task done
// sub task 2----- begin
// async block done
// main task done
fn main3() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(
        // 一个异步块，一个调度单位
        async {
            println!("async block start");
            println_current_time();

            // 生成一个调度单位
            tokio::spawn(
                // 阻塞任务，不会让出调度任务
                async {
                    println!("sub task begin");
                    println_current_time();
                    thread::sleep(Duration::from_secs(3));
                    println!("sub task done");
                    println_current_time();
                    return;
                },
            );

            tokio::spawn(async {
                println!("sub task 2----- begin");
                println_current_time();
                // 让出调度任务
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("sub task 2----- done");
                println_current_time();
            });

            println!("async block sleep");
            println_current_time();
            // continuation， 让出调度任务
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("async block done");
            println_current_time();
        },
    );

    println!("main task done");
    println_current_time();
}

/// 运行结果
/// async block start
/// async block sleep
/// async block done
/// main task done
fn main2() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(
        // 一个异步块，一个调度单位
        async {
            println!("async block start");
            println_current_time();

            // 生成一个调度单位
            tokio::spawn(
                // 阻塞任务，不会让出调度任务
                async {
                    println!("sub task begin");
                    println_current_time();
                    thread::sleep(Duration::from_secs(3));
                    println!("sub task done");
                    println_current_time();
                    return;
                },
            );

            tokio::spawn(async {
                println!("sub task 2----- begin");
                println_current_time();
                // 让出调度任务
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("sub task 2----- done");
                println_current_time();
            });

            println!("async block sleep");
            println_current_time();
            // continuation， 让出调度任务
            // tokio::time::sleep(Duration::from_secs(5)).await;
            println!("async block done");
            println_current_time();
        },
    );

    println!("main task done");
    println_current_time();
}

use chrono::Local;

fn println_current_time() {
    // 获取本地时间
    let now = Local::now();
    // println!("Current time: {}", now);

    // 自定义格式
    println!("Formatted: {}\n", now.format("%Y-%m-%d %H:%M:%S"));

    // UTC 时间
    // let utc_now = chrono::Utc::now();
    // println!("UTC time: {}", utc_now);
}
