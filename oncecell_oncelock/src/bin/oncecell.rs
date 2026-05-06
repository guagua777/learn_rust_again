use std::cell::OnceCell;

// 只能设置一次，或者说只能初始化一次
fn main() {
    test_oncecell();
    return;
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let r = cell.get_or_init(|| {
        String::from("hello")
    });
    assert_eq!(r, "hello");
    assert!(cell.get().is_some());
}


fn test_oncecell() {
    // 获取oncecell的可变引用，修改值为world
    let mut cell = OnceCell::new();
    let _ = cell.set(String::from("hello"));

    // 要使用get_mut()，而不是get
    if let Some(value) = cell.get_mut() {
        *value = "world".to_string();
    }

    // 再次设置值，不会生效
    let r = cell.set(String::from("new value"));

    assert!(r.is_err());

    if let Some(value) = cell.get() {
        println!("{}", value);
    }
}