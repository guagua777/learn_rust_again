use std::sync::mpsc;
use std::thread;


type Task = Box<dyn FnOnce() + Send + 'static>;

fn hello() {
    println!("Hello, world!");
}


fn main() {
    let (tx, rx) = mpsc::channel::<Task>();

    let h = thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            task();
        }
    });

    let task = || println!("closure Hello, world!");

    tx.send(Box::new(hello)).unwrap();
    tx.send(Box::new(task)).unwrap();
    tx.send(Box::new(|| println!("inner closure hello"))).unwrap();

    h.join().unwrap();



}
