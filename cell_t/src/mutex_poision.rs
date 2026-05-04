use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn main() {
    // 需要多个引用
    // 需要共享引用
    let data = Arc::new(Mutex::new(5));
    {
        let loacl_data = data.clone();
        thread::spawn(move || {
            let mut data = loacl_data.lock().unwrap();
            *data = 100;
            panic!();
        }).join().unwrap_err(); //.unwrap();
    }

    {
        let local_data = data.clone();
        thread::spawn(move || {
            match local_data.lock() {
                Ok(mut data) => {
                    *data = 10000;
                }
                Err(poisioned) => {
                    println!("thread2 poisioned:{poisioned:#?}");
                    let mut data = poisioned.into_inner();
                    *data = 20000;
                    println!("thread2 new data:{data:#?}");
                }
            }
        }).join().unwrap();
    }
}