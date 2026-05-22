// use add_one;

// 声明模块
mod adder_sub;
// 使用模块
use adder_sub::adder_sub;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    // 打印当前的路径
    println!("Current path: {:?}", std::env::current_dir());
}


// 模块也是先声明，后使用