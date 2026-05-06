use std::{sync::atomic::AtomicUsize, thread, time::Duration};
use std::sync::atomic::Ordering;



fn main() {
    let done = AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..10 {
            s.spawn( || {
                for i in 0..100 {
                    thread::sleep(Duration::from_millis(20));
                    // let value = done.load(Ordering::Relaxed);
                    // done.store(value + 1, Ordering::Relaxed);

                    done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }


        loop {
            let value = done.load(Ordering::Relaxed);
            if value == 1000 {
                break;
            }
            println!("process: {value}/1000");
            thread::sleep(Duration::from_millis(100));
        }
    });


    println!("all done");


    
}
