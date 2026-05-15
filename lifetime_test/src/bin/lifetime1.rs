

fn main() {
    println!("Hello, world!");
}


// foo 的生命周期为 'a 
// 或者说是，不能超过 'a 生命周期的范围
// 因为'a的值是由多个 条件组成的
// 例如：
// let 'a = lifetime1； 条件1
// 'a = 'a 相交 lifetime2； 条件2，最终值为：两者交集
// 'a = 'a 相交 lifetime3； 条件3，最终值为：三者交集
// foo<'a>
// // `foo` has a lifetime parameter `'a`

// foo<'a, 'b>
// // `foo` has lifetime parameters `'a` and `'b`

// 作用于引用
// the borrow checker uses explicit lifetime annotations to determine how long references should be valid