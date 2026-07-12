// ❌ 错误：返回引用局部变量的闭包
fn bad_closure() -> impl Fn() {
    let local = String::from("temp"); // 短期局部变量
    // 闭包引用捕获 local，绑定 local 的短生命周期
    || println!("{}", local)
} // local 在这里销毁，闭包持有无效引用 → 编译报错

fn main() {
    let c = bad_closure();
    c();
}