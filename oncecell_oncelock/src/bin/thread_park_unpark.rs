use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
use std::thread;


fn main() {
    let flag = Arc::new(AtomicBool::new(false));
    // let flag2 = flag.clone();
    let flag2 = Arc::clone(&flag);

    let thread_park = thread::spawn(move || {
        while !flag2.load(Ordering::Relaxed) {
            println!("parking thread");
            thread::park();
            println!("unparking thread");
        }
        println!("flag received")
    });

    thread::sleep(Duration::from_secs(1));
    flag.store(true, Ordering::Relaxed);


    // unpark
    thread_park.thread().unpark();
    thread_park.join().unwrap();
}