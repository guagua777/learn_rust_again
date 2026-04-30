// 自定义容器
#[derive(Debug)]
struct MyContainer<T> {
    inner: Vec<T>,
}

// 给 MyContainer 简单实现个 new
impl<T> MyContainer<T> {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }
}


use std::iter::FromIterator;

impl<T> FromIterator<T> for MyContainer<T> {


    // fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
    // 从迭代器构造 MyContainer
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // 把迭代器元素收集进 Vec，再包一层
        let inner: Vec<T> = iter.into_iter().collect();
        MyContainer { inner }
    }
}


#[test]
fn main1() {
    let nums = [1, 2, 3, 4];

    // 直接 collect 成 MyContainer<i32>
    let container: MyContainer<i32> = nums.iter().copied().collect();

    println!("{:#?}", container);
}