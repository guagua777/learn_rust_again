use core::time;
use std::sync::Arc;
use std::{thread, vec};

#[test]
pub fn cross_thread() {
    let s = Arc::new(String::from("hello"));
    let s_clone = Arc::clone(&s);
    let handler = thread::spawn(move || println!("cross_thread ------ : {}", s_clone));
    println!("main thread: {}", s);
    // println!("main thread: {}", s_clone);
    handler.join().unwrap();
}


#[test]
pub fn cross_thread1() {
    let vec = vec![1, 2, 3, 4];
    let vec = Arc::new(vec);
    let vec_clone = Arc::clone(&vec);
    let handler = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(3));
        println!("cross_thread ------ : {:?}", vec_clone);
    });
    println!("main thread: {:?}", vec);
    // println!("main thread: {}", s_clone);
    // handler.join().unwrap();
}


// #[test]
// pub fn cross_thread2() {
//     let vec = vec![1, 2, 3, 4];
//     let vec = Arc::new(vec);
    
//     let vec_clone = Arc::clone(&vec);
//     // 同样会报错
//     let vec_clone = vec_clone.chunks(4);
//     let handler = thread::spawn(move || {
//         thread::sleep(time::Duration::from_secs(3));
//         println!("cross_thread ------ : {:?}", vec_clone);
//     });
//     println!("main thread: {:?}", vec);
// }



