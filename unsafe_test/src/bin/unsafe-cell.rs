fn main() {
    use std::cell::UnsafeCell;
    let uc = UnsafeCell::new(100);
    // 手动裸指针访问，需要程序员自己保证借用安全
    // *uc.get()相当于c语言的 *p，即获取内存中的值
    let val = unsafe { *uc.get() };
    unsafe { *uc.get() = 200 };

    println!("{}", unsafe { *uc.get() });
}
