use std::{sync::Mutex, thread};

// move 闭包的行为： 在循环中，每次迭代执行 thread::spawn(move || { ... }) 时：

// move 关键字会将 data 的所有权（或引用的所有权）移动到闭包中
// 由于 &mut [i32; 5] 不实现 Copy trait，第一次迭代后 data 就被"消耗"了
// 第二次迭代时，data 已经失效，无法再次移动

//这个错误的根本原因是 move 闭包会转移变量的所有权，但 data 是一个不实现 Copy trait 的引用类型。

fn leak() {
    // let data: &mut [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));
    // let data: &'static mut [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));
    let data: &'static [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));
    
    let mut handlers = Vec::new();

    for _ in 0..10000 {
        // data是个引用， move 的是 data的引用
        let h = thread::spawn(move || {
            println!("{data:#?}");
        });
        handlers.push(h);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());


}