use std::sync::{Mutex, Condvar};
use std::thread;

fn main() {
    // 共享状态 + 锁 + 条件变量
    let pair = (Mutex::new(false), Condvar::new());
    let (lock, cvar) = &pair;

    // 等待线程
    let t = thread::spawn(move || {
        let mut guard = lock.lock().unwrap();
        // ✅ 循环防止虚假唤醒！
        while !*guard {
            // 释放锁并等待，醒来后重新上锁
            guard = cvar.wait(guard).unwrap();
        }
        println!("收到通知，条件满足，继续执行！");
    });

    // 主线程修改状态并发通知
    thread::sleep(std::time::Duration::from_millis(500));
    let mut guard = lock.lock().unwrap();
    *guard = true; // 修改共享条件
    cvar.notify_one(); // 唤醒等待线程
    drop(guard);

    t.join().unwrap();
}