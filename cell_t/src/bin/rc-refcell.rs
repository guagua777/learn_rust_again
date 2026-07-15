use std::rc::Rc;
use std::cell::RefCell;

// 定义数据类型
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // 创建：Rc包裹 RefCell，初始数据
    let data: Rc<RefCell<Person>> = Rc::new(
        RefCell::new(Person {
            name: String::from("Alice"),
            age: 20,
        })
    );

    // 克隆Rc，产生多个所有者（只是增加引用计数，不是拷贝原数据）
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    // 通过 owner1 修改原始数据
    owner1.borrow_mut().age = 25;
    owner1.borrow_mut().name = String::from("Alicia");

    // 通过 owner2 读取，看到的是同一份修改后的数据
    println!("owner2 view: {:?}", owner2.borrow());
    // 原始变量读取，也是同一份数据
    println!("original view: {:?}", data.borrow());

    // 查看引用计数
    println!("Rc strong count: {}", Rc::strong_count(&data)); // 3

    // 函数接收 Rc<RefCell<T>>，同样可以修改
    fn add_year(p: &Rc<RefCell<Person>>) {
        p.borrow_mut().age += 1;
    }
    add_year(&owner2);
    println!("after add_year: {:?}", data.borrow());
}