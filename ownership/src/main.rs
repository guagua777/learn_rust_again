fn main() {
    println!("Hello, world!");

    let s1 = String::from("a");
    let s2 = s1; // s1 标记为 moved
    // println!("{}", s1); // 报错
    let s1 = String::from("b"); // 重新绑定，s1 恢复可用
    println!("{}", s1); // 正常
}
