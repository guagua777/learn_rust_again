use std::mem::size_of;

fn main() {
    println!("size of &i32: {}", size_of::<&i32>());
    println!("size of Option<&i32>: {}", size_of::<Option<&i32>>());
    // 64位系统：两者都是 8 字节
    println!("size of Option<i32>: {}", size_of::<Option<i32>>());
    // 普通 Option<i32> 是 8 字节（4+标记），和上面有区别


       println!("size of Option<i32>: {}", size_of::<Option<i64>>());
}