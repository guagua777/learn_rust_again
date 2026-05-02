use std::time::Instant;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};



// static mut COUNTER: usize = 0;
static COUNTER: AtomicUsize = AtomicUsize::new(0);


fn main() {
    let start = Instant::now();

    let mut handlers = Vec::new();

    for _ in 0..1000 {
        let h = thread::spawn(|| {
            for _ in 0..1000 {
                // unsafe {
                //     COUNTER += 1;
                // }
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        });
        handlers.push(h);
    }


    handlers.into_iter().for_each(|h| h.join().unwrap());
    // println!("COUNTER: {}", unsafe { COUNTER });
    println!("COUNTER: {}", COUNTER.load(Ordering::Relaxed));


    let duration = start.elapsed();
    println!("Execution time: {:?}", duration.as_micros());
}