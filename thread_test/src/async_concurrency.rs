use std::time::Duration;

#[test]
fn main1() {
    trpl::run(
        // async块
        async {
            // trpl::spawn_task() 启动一个异步任务，任务在async块中执行
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                // trpl::sleep，而不是std::thread::sleep
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}


#[test]
fn main2() {
    trpl::run(
        // async块
        async {
        // trpl::spawn_task() 启动一个异步任务，任务在async块中执行
        let handle = trpl::spawn_task(async {
        for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();


    });
}




#[test]
fn main3() {
    trpl::run(
        // async块
        async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;


    });
}





#[test]
fn main4() {
    trpl::run(
        // async块
        async {

        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");



    });
}

#[test]
fn main5() {
    trpl::run(
        // async块
        async {

        let (tx, mut rx) = trpl::channel();
        
        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;

    });
}




