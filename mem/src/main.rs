use std::mem;

fn main() {
    let mut x = String::from("旧值");
    let old = mem::replace(&mut x, String::from("新值"));
    println!("旧值:{}", old); // 旧值:旧值
    println!("x:{}", x); // x:新值
}
