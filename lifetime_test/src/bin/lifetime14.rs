// 错误示例：返回引用捕获局部变量的闭包，存在生命周期问题
fn bad_closure() -> impl Fn() {
    let local_str = String::from("临时局部字符串");

    // ❌ 默认引用捕获，闭包持有 &String，绑定 local_str 的短生命周期
    let closure = || {
        println!("{}", local_str);
    };

    closure
    // local_str 函数结束时销毁，闭包带着无效引用 → 编译报错
}

fn main() {
    let my_closure = bad_closure();
    my_closure();
}