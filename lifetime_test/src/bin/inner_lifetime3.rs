fn main() {
    let data = String::from("hello");
    let ref1: &String = &data; // 生命周期绑定在 data 上
    // 语义层面相当于这样
    // let ref2: &'a String = &'a data; // 生命周期绑定在 data 上，和 ref1 一模一样
    // 到这时，'a 已经结束，所以 ref2 也不能再使用

    // 假设 data 被某种方式"部分失效"但内存仍在
    // 比如把 data 移走：
    let moved_data = data; // data 失去所有权
    // println!("{}", ref1);          // 编译错误！ref1 引用的 data 已经不存在了
    //                                 即使 String 的内存可能还在
}

fn test_lifetime<'a>() {
    let data = String::from("hello");
    let ref1: &String = &data; // 生命周期绑定在 data 上
    // 语义层面相当于这样
    // let ref2: &'a String = &'a data; // 生命周期绑定在 data 上，和 ref1 一模一样
    // 到这时，'a 已经结束，所以 ref2 也不能再使用

    // 假设 data 被某种方式"部分失效"但内存仍在
    // 比如把 data 移走：
    let moved_data = data; // data 失去所有权
    println!("{}", ref1);
}
