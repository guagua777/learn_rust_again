use std::any::Any;

fn main() {
    // 把不同类型装进 Box<dyn Any>
    let any1: Box<dyn Any> = Box::new(123i32);
    let any2: Box<dyn Any> = Box::new("hello world".to_string());
    let any3: Box<dyn Any> = Box::new(("a", 99));

    // 1. downcast 向下转换（返回 Result，安全）
    if let Ok(num) = any1.downcast::<i32>() {
        println!("i32 值: {}", num);
    }

    // 2. downcast_ref 获取不可变引用
    if let Some(s) = any2.downcast_ref::<String>() {
        println!("String: {}", s);
    }
}