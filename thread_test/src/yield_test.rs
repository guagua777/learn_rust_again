use std::{ffi::os_str::Display, thread, time::{Duration, Instant}};

fn slow(name: &str, ms: u64) {
    // 线程池的sleep
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}


#[test]
fn main1() {
    trpl::run(
        async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            // trpl的sleep
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        // 如果没有这个await，a和b都不会执行。
        trpl::select(a, b).await;
        }
    );
}





#[test]
fn main3() {
    trpl::run(
        async {
            // let one_ns = Duration::from_nanos(1);
            let one_ns = Duration::from_secs(1);

            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    println!("sleep正在睡眠...");        
                    trpl::sleep(one_ns).await;
                }
            }.await; 
            // };//.await; // 此处不加await，则这个任务不执行。
            let time = Instant::now() - start;

            println!("'sleep'版本在{}秒后完成。", time.as_secs_f32());


            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    // println!("yield_now正在yield...");
                    trpl::yield_now().await;
                }
            }.await;
            let time = Instant::now() - start;

            println!("'yield_now'版本在{}秒后完成。", time.as_secs_f32());
        }
    );
}

