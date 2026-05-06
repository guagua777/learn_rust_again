use std::sync::{Arc, Condvar, Mutex};
use std::thread;



// 主线程 等待 子线程完成某个动作，并且在等待期间，不消耗cpu
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {

        // 报错原因：
        // Arc 是引用计数智能指针，多个 Arc 实例共享同一个值的所有权
        // 当你写 *pair2 时，Rust 会尝试将内部的元组移动出来（获取所有权）
        // 但这是不允许的！因为 Arc 的设计就是为了共享所有权，其他 Arc 实例可能还在使用这个值
        // 而且 (Mutex<bool>, Condvar) 这个元组没有实现 Copy trait，无法直接复制

        // Arc 的内容只能借用，不能移出所有权。
        // let (mutex, cond) = *pair2;
        // 获取arc内部的引用，不会移出所有权。
        let (mutex, cond) = &*pair2;
        let mut lock = mutex.lock().unwrap();
        *lock = true;
        cond.notify_one();
    });
    

    // 主线程等待
    let (mutex, cond) = &*pair;
    let mut lock = mutex.lock().unwrap();
    // 当lock为false，则等待
    while !*lock {
        // lock 需要重新赋值
        lock = cond.wait(lock).unwrap();
    }
    println!("子线程完成动作");
}
