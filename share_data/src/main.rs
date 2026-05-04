use std::thread;


static DATA: [i32; 5] = [1, 2, 3, 4, 5];


fn main() {
    let mut handlers = Vec::new();
    for _ in 0..100 {
        let h = thread::spawn(|| {
            println!("{DATA:#?}");
        });
        handlers.push(h);
    }

    // for h in handlers {
    //     h.join().unwrap();
    // }

    handlers.into_iter().for_each(|h| h.join().unwrap());
}
