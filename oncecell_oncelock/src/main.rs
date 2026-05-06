use std::cell::OnceCell;


fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let r = cell.set(String::from("hello"));
    assert!(r.is_ok());

    let r = cell.set(String::from("world"));
    assert!(r.is_err());
}
