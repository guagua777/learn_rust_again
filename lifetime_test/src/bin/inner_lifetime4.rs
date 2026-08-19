use std::sync::{Arc, RwLock};

fn main1() {
    let arc = Arc::new(RwLock::new(String::from("hello")));

    let guard = arc.read().unwrap();
    // guard: RwLockReadGuard<'a, String>
    // 这里的 'a 实际上是从 RwLock 的内部借用出来的生命周期

    // 假设你想把引用存起来
    // let saved_ref: &String = &*guard;
    // 然后 guard 被 drop → 'a 结束 → saved_ref 变成"悬垂引用"（类型层面）

    drop(guard);
    // 此时 arc 仍然持有数据，数据确实还在
    // 但 Rust 类型系统不允许你持有跨越 guard drop 的引用
}

fn main() {
    test_lifetime();
}

fn test_lifetime() -> &String {
    // "hello" --> RwLock("hello") ---> Arc(RwLock("hello"))
    let arc = Arc::new(RwLock::new(String::from("hello")));
    let guard = arc.read().unwrap();
    // guard: RwLockReadGuard<'a, String>
    // 这里的 'a 实际上是从 RwLock 的内部借用出来的生命周期

    // 假设你想把引用存起来
    let saved_ref = &*guard;
    // 重点是'a 结束
    // 然后 guard 被 drop → 'a 结束 → saved_ref 变成"悬垂引用"（类型层面）

    drop(guard);
    // 此时 arc 仍然持有数据，数据确实还在
    // 但 Rust 类型系统不允许你持有跨越 guard drop 的引用
    // 跨越 guard drop 不可以
    saved_ref
}
