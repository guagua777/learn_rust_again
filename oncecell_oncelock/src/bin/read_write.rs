fn main() {
    let mut a = 5;
    a = 10;
    println!("{}", a);
    a = 20;
    println!("{}", a);
    let c = a;
    let d = a;
    println!("{}", c);
    println!("{}", d);
}

// 读和写不会转移所有权
// 赋值才会转移所有权，赋值给别的变量，才会转移所有权