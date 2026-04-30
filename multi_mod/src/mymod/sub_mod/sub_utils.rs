
// 同层模块间应用用crate
// 二进制引用库 包名::模块名
// 为什么两者不一样？
use crate::mymod::utils::add;



pub fn sub_add(a: i32, b: i32) -> i32 {
    add(a, b)
}