fn main() {
    let mut x = 10;
    let mut r1 = &mut x;
    let r2 = &mut *r1; // 可变再借用
    *r2 += 1;
    // NLL 生效后 r2 用完，r1 恢复可用

    println!("{}", r1);
}
