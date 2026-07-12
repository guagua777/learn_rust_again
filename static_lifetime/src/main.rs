use std::fmt::Display;

// 泛型约束 T: 'static：只要求类型没有短期外部借用
fn consume_static_type<T: 'static + Display>(val: T) {
    // val 是符合 'static 约束的自持类型，但可以随时销毁
    println!("{}", val);
}

fn main() {
    let s = String::from("临时字符串"); // String: 'static ✅
    consume_static_type(s);
    // 字符串已经被移动并销毁，并没有活到程序末尾 ✅

    // 静态字面量引用（真实 &'static str）
    let literal: &'static str = "全局字面量";
    println!("{}", literal);
    // 这个底层字面量数据确实存在静态内存，存活到程序结束
}