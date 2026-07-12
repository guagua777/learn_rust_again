fn main() {
    use std::mem;

    let mut s = String::from("hello");
    let old = mem::take(&mut s);
    println!("old:{}", old); // old:hello
    println!("s:{}", s); // s: (空字符串，String默认值)
}
