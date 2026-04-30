// 引用库代码
use multi_mod::hello;
use multi_mod::mymod::utils::add;

fn main() {
    hello();
    println!("1+2={}", add(1, 2));
    println!("This is app1!");
}