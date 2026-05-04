use std::{sync::{Arc, Mutex}, thread};

fn leak() {

    let data= Arc::new([1, 2, 3, 4, 5]);
    
    let mut handlers = Vec::new();

    for _ in 0..10000 {
        // 每次clone，引用数+1
        // 传递的是clone后的Arc，不会消耗原始的Arc
        let local_data = data.clone();
        let h = thread::spawn(move || {
            println!("{local_data:#?}");
        });
        handlers.push(h);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());


}