fn main() {
    println!("Hello, world!");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}",hello);
    println!("{}",world);


}

// slice为fat pointer，包含了指向数据的指针和长度信息
// 字符串的切片，类型为&str
// 
