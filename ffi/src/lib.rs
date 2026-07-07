// lib.rs
use libc::c_int;

// 导出给C调用
#[unsafe(no_mangle)] // 禁止Rust名字混淆，导出原始函数名
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}


#[repr(C)]
pub struct Context {
    // 内部复杂Rust结构，不暴露给C
    data: Vec<u8>,
}

// 只返回指针，C看不到内部
#[unsafe(no_mangle)]
pub extern "C" fn ctx_new() -> *mut Context {
    // Box::new就是把它封装一下
    let ctx = Box::new(Context { data: Vec::new() });
    Box::into_raw(ctx)
}

#[unsafe(no_mangle)]
pub extern "C" fn ctx_free(p: *mut Context) {
    if !p.is_null() {
        unsafe { Box::from_raw(p); }
    }
}
