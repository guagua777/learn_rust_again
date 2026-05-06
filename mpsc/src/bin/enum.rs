use std::sync::mpsc;
use std::thread;


type Task = Box<dyn FnOnce() + Send + 'static>;

fn hello() {
    println!("Hello, world!");
}

enum Msg {
    Call(Task),
    Exit,
}


fn main() {
    let (tx, rx) = mpsc::channel::<Msg>();

    let h = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Msg::Call(task) => task(),
                Msg::Exit => break,
            }
        }
    });

    let task = || println!("closure Hello, world!");

    tx.send(Msg::Call(Box::new(hello))).unwrap();
    tx.send(Msg::Call(Box::new(task))).unwrap();
    tx.send(Msg::Call(Box::new(|| println!("inner closure hello")))).unwrap();
    tx.send(Msg::Exit).unwrap();    

    h.join().unwrap();



}
