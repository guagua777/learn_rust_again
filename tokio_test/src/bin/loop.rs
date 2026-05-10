use std::{thread, time::Duration};

// fn main1() {
//     loop {
//         println!("hello world");
//         let mut i = 0;
//         loop {
//             println!("i: {}", i);
//             i += 1;
//             if i > 10 {
//                 // return;
//                 break;
//             }
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         thread::sleep(Duration::from_secs(1));
//         println!("sub task done");
//         return;
//     });


//     println!("main task begin");
//     thread::sleep(Duration::from_secs(5));
//     println!("main task done");

// }

use tokio::runtime;

fn main() {
     let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
    // rt.spawn(async {
        thread::sleep(Duration::from_secs(1));
        println!("sub task done");
        return;
    });

   
    println!("main task begin");
    thread::sleep(Duration::from_secs(3));
    println!("main task done");

}

