fn take_static_ref(r: &'static i32) {
    println!("static ref: {}", r);
}

// static 全局变量：数据存静态内存，存活整个程序
static GLOBAL_NUM: i32 = 999;

fn main() {
    take_static_ref(&GLOBAL_NUM); // ✅ &'static i32，符合签名

    let local_num = 123;
    // take_static_ref(&local_num); 
    // ❌ 编译报错！&local_num 是局部生命周期，不是 'static
    // 这个函数严格要求引用本身必须是 'static 生命周期
}