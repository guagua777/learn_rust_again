use std::cell::Cell;
// 外面是不可变引用，但是要修改里面的值的时候，使用Cell
fn main() {
    let cell = Cell::new(5);
    assert_eq!(cell.get(), 5);

    assert_eq!(cell.replace(10), 5);
    assert_eq!(cell.get(), 10);

    let ten = cell.into_inner();
    assert_eq!(ten, 10);
    // cell.get(); 报错，因为cell已经被移动了


    let cell = Cell::new(String::from("hello"));
    // 不是cell.get
    // String 没有实现 Copy trait，所以不能用 cell.get() 来获取值
    assert_eq!(cell.take(), "hello");
    assert_eq!(cell.take(), String::default());


    cell.set(String::from("world"));
    assert_eq!(cell.take(), "world");
}
