#[derive(Debug)]
struct MyBag<T> {
    count: usize,
    data: Vec<T>,
}

impl<T> MyBag<T> {
    fn empty() -> Self {
        Self {
            count: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.count += 1;
    }
}



use std::iter::FromIterator;

impl<T> FromIterator<T> for MyBag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // 1. 把传入的可迭代对象转成迭代器
        let mut it = iter.into_iter();
        
        // 2. 创建空容器
        let mut bag = MyBag::empty();

        // 3. 手动逐个取元素、装入容器
        while let Some(item) = it.next() {
            bag.push(item);
        }

        // 4. 返回构造好的容器
        bag
    }
}


#[test]
fn main1() {
    let arr = [10, 20, 30, 40];

    // 方式1：变量标注类型
    let bag1: MyBag<i32> = arr.iter().copied().collect();
    println!("bag1 = {:#?}", bag1);

    // 方式2：涡轮鱼语法 ::<> 不用标变量类型
    let bag2 = arr.iter().copied().collect::<MyBag<i32>>();
    println!("bag2 = {:#?}", bag2);
}