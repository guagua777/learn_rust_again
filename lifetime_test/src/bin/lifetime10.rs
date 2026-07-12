// 带显式生命周期参数的引用结构体：只借用外部数据，不拥有内存
#[derive(Debug)]
struct LocalView<'a> {
    message: &'a str,
}

fn main() {
    // 原始数据，存活于整个 main 作用域
    let original = String::from("hello local view");

    // ✅ 局部使用：生命周期完全匹配，零开销
    let view = LocalView {
        message: &original,
    };
    println!("引用结构体: {:?}", view);

    // ====== 错误示范（取消注释会编译报错）======
    // let bad_view: LocalView;
    // {
    //     let short_lived = String::from("临时字符串");
    //     bad_view = LocalView { message: &short_lived };
    // } // short_lived 在这里被销毁
    // println!("{}", bad_view.message); // 悬垂引用，编译不通过
}