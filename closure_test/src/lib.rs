fn main1() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}


fn main2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}


fn main3() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // 报错
    println!("After calling closure: {list:?}");

    borrows_mutably();
    println!("After calling closure: {list:?}");
}


use std::thread;

fn main4() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // 移动到新线程中
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main5() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key 被定义为接收一个 FnMut 闭包的原因是它会多次调用这个闭包
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}



// 一个会将捕获的值从闭包体中移出的闭包只会实现 FnOnce trait
fn main6() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    // 里面的闭包，实现了FnOnce
    // 尝试在 sort_by_key 上使用一个 FnOnce 闭包
    // 一个只实现了 FnOnce trait 的闭包的例子，因为它从环境中移出了一个值

    // 该代码试图通过将闭包环境中的 value（一个 String）插入 sort_operations vector 来实现计数。
    // 闭包捕获了 value，然后通过将 value 的所有权转移给 sort_operations vector 的方式将其移出闭包。

    // 这个闭包只能被调用一次；尝试第二次调用它将无法工作，
    // 因为这时 value 已经不在闭包的环境中，无法被再次插入 sort_operations 中！因而，这个闭包只实现了 FnOnce。

    // 第二次调用时，环境中已经不存在 value 了，所以会报错。
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}


fn main7() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}