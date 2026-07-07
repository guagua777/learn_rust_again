use libc::{c_int, c_char};

// 声明C标准库puts
unsafe extern "C" {
    fn puts(s: *const c_char) -> c_int;
}   

fn main() {
    let c_str = b"hello ffi\0".as_ptr() as *const c_char;
    unsafe {
        puts(c_str); // 外部函数调用必须unsafe
    }
}