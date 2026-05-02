use std::time::Instant;
use std::thread;

static mut COUNTER: usize = 0;


fn main() {
    let start = Instant::now();

    let mut handlers = Vec::new();

    for _ in 0..1000 {
        let h = thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handlers.push(h);
    }


    handlers.into_iter().for_each(|h| h.join().unwrap());
    println!("COUNTER: {}", unsafe { COUNTER });


    let duration = start.elapsed();
    println!("Execution time: {:?}", duration.as_micros());
}