use std::thread;


#[test]
fn main1() {
    let mut message = String::from("主线程数据");

    // ✅ 开启线程作用域
    thread::scope(|s| {
        // 子线程1：只读借用
        s.spawn(|| {
            println!("子线程1：{}", message);
        });

        // 子线程2：可变借用（Rust 会自动保证安全，不会同时读写）
        s.spawn(|| {
            // message.push_str(" + 子线程修改");
            println!("子线程2：{}", message);
        });
    }); 
    // 👆 到这里，scope 会自动等待所有子线程结束！

    println!("主线程最终：{}", message);
}



// #[test]
// fn main2() {
//     let mut message = String::from("主线程数据");

//     let mut sum_message = String::from("");

//     // ✅ 开启线程作用域
//     thread::scope(|s| {
//         // 子线程1：只读借用
//         let h1 = s.spawn(|| {
//             println!("子线程1：{}", message);
//             message
//         });

//         // 子线程2：可变借用（Rust 会自动保证安全，不会同时读写）
//         let h2 = s.spawn(|| {
//             message.push_str(" + 子线程修改");
//             println!("子线程2：{}", message);
//             message
//         });


//     }); 
//     // 👆 到这里，scope 会自动等待所有子线程结束！

//     println!("主线程最终：{}", message);
// }