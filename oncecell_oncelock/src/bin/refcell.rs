use std::cell::RefCell;

fn main() {
    // RefCell包裹任意类型（如String）
    let r = RefCell::new(String::from("hello"));

    // 多个不可变借用（允许）
    let r1 = r.borrow();
    let r2 = r.borrow();
    println!("{} {}", r1, r2); // hello hello

    // 不可变借用结束后，可变借用（允许）
    drop(r1);
    drop(r2);
    let mut w = r.borrow_mut();
    w.push_str(" world");
    println!("{}", w); // hello world

    // 违反规则：同时可变+不可变借用（运行时panic）
    // let r3 = r.borrow();
    // let mut w2 = r.borrow_mut(); // 线程panic
}