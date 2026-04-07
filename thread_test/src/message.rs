use std::sync::mpsc;


// 多个生产者，一个消费者
fn message() {
    let (tx, rx) = mpsc::channel::<i32>();
}



use std::thread;

fn main1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // 发送消息到通道
        // 使用线程
        tx.send(val).unwrap();
    });

    // 接收端
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}


fn main2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 编译报错
        // println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}





use std::time::Duration;

fn main3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn main4() {
    let (tx, rx) = mpsc::channel();

    // clone 
    // 多个发送者
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --snip--
}

