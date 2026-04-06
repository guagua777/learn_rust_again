fn main() {
    println!("Hello, world!");
}

// Person 结构体，生命周期，不能超过 'a 生命周期的范围
// 'a 的生命周期为name字段的生命周期
// Person 结构体，生命周期，不能超过 name 生命周期的范围
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 如果Person的生命周期，超过了name的生命周期，就会出现悬垂引用