use std::any::{Any, TypeId};

// &dyn Any
fn check_type(val: &dyn Any) {
    match val.type_id() {
        t if t == TypeId::of::<i32>() => println!("这是i32"),
        t if t == TypeId::of::<String>() => println!("这是String"),
        _ => println!("未知类型"),
    }
}

fn main() {
    let num = 100;
    check_type(&num);
    let s = String::from("test");
    check_type(&s);
}