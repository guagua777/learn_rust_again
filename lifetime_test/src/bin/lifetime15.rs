// ✅ move 把 String 所有权移入闭包，闭包生命周期独立
fn good_closure_owned() -> impl Fn() + 'static {
    let local_str = String::from("自有所有权字符串");

    // move 将整个 String 移动进闭包，不再是引用
    let closure = move || {
        println!("{}", local_str);
    };

    closure
}

fn main() {
    let my_closure = good_closure_owned();
    my_closure();
    my_closure(); // 可多次调用，生命周期不受外部变量限制
}