use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone)]
struct SharedMsg {
    message: Arc<String>, // 引用计数，多线程安全共享
}

fn main() {
    let shared = SharedMsg {
        message: Arc::new(String::from("hello arc shared")),
    };

    // ✅ 跨线程传递
    let clone_msg = shared.clone();
    let handle = thread::spawn(move || {
        println!("子线程: {:?}", clone_msg);
    });
    handle.join().unwrap();

    println!("主线程: {:?}", shared);
}