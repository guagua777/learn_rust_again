#[derive(Debug)]
struct OwnedMsg {
    message: String, // 拥有完整内存所有权，生命周期独立
}

fn main() {
    // 构造自有版本
    let owned = OwnedMsg {
        message: String::from("hello owned string"),
    };
    println!("String版本结构体: {:?}", owned);

    // ✅ 可以存入容器、自由移动、脱离原数据源
    let mut vec = Vec::new();
    vec.push(owned);
    println!("存入Vec: {:?}", vec);
}