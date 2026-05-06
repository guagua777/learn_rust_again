use std::{sync::atomic::{AtomicUsize, Ordering}, thread,};



fn main() {
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..1000 {
            s.spawn( || {
                inc(&counter);
            });
        }
    });


    println!("counter: {}", counter.load(Ordering::Relaxed));
}

fn inc(counter: &AtomicUsize) {
    let mut current = counter.load(Ordering::Relaxed);
    
    loop {
        let next = current + 1;
        match counter.compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => {
                println!("current: {}, v: {}", current, v);
                // 将新的值赋值给current
                current = v;
            }
        }
    }
    
}