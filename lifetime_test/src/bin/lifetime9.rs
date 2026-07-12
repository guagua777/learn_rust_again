fn long<'a>(x: &'a mut i32) {}
fn main() {
    let mut val = 1;
    // 短生命周期引用可以传入需要长生命周期的函数 ✅
    long(&mut val);
}