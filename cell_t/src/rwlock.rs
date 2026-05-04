// send: 可以跨线程传输
// sync: 可以在多个线程之间安全的共享引用
use std::sync::{Arc, RwLock};
use std::thread;

#[test]
fn main1() {
    let counter = Arc::new(RwLock::new(0));

    let mut handlers = Vec::new();

    for _ in 0..10 {
        let counter = counter.clone();
        let h = thread::spawn(move || {
            let counter = counter.read().unwrap();
            println!("counter: {counter}");
        });
        handlers.push(h);
    }


    {
        let counter = counter.clone();
        let h = thread::spawn(move || {
            let mut counter = counter.write().unwrap();
            *counter += 1;
            println!("writer counter: {counter}");
        });
        handlers.push(h);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    println!("counter end is: {}", counter.read().unwrap());
}