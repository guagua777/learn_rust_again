rustc --help | grep -i crate-type


# Rust FFI 完整介绍
FFI = Foreign Function Interface，**外部函数接口**，作用是让 Rust 和其他语言（C/C++、Python、Go、Java 等）互相调用：
1. Rust 调用 C 动态库/静态库
2. 把 Rust 编译成 C ABI 库，给 C/C++/其他语言调用

Rust FFI 核心基准：**C ABI**，所有跨语言交互都基于 C 的调用约定、内存模型。

## 一、基础核心概念
### 1. ABI（Application Binary Interface）
- Rust 自身没有稳定 ABI，只有 `extern "C"` 块会使用标准 C ABI；
- 不加 `extern "C"` 的函数不能给 C 调用，二进制布局不稳定；
- `extern "system"`：Windows 下等价 `extern "stdcall"`，跨平台 Windows API 使用。

### 2. 不安全边界 `unsafe`
FFI 是 Rust 安全体系的**逃逸口**：
- 外部 C 函数不受 Rust 所有权、借用检查约束；
- 裸指针、空指针、越界、生命周期错误都不会被编译器拦截；
- 所有跨语言操作必须包裹 `unsafe`。

### 3. 类型映射（Rust ↔ C）
| C 类型 | Rust 对应类型 | 说明 |
|--------|--------------|------|
| `void*` | `*mut c_void` / `*const c_void` | 通用裸指针 |
| `char` | `c_char` | i8/u8，平台相关 |
| `short` | `c_short` | i16 |
| `int` | `c_int` | i32 |
| `long` | `c_long` | 32/64位平台长度不同 |
| `long long` | `c_longlong` | i64 |
| `float/double` | `f32/f64` | 直接对应 |
| `uint8_t`… | `u8/u16/u32/u64` | 固定宽度无符号 |
| `bool` | `c_bool` | C99 bool，非 Rust bool |
| `char*` | `*mut c_char` | 以 `\0` 结尾 C 字符串 |

> 重要：Rust `&str`/`String` 不能直接传给 C，必须转成 C 零结尾字符串。

## 二、场景1：Rust 调用 C 代码
### 1. 声明外部 C 函数
用 `extern "C"` 块声明 C 库导出函数：
```rust
use libc::{c_int, c_char};

// 声明C标准库puts
extern "C" {
    fn puts(s: *const c_char) -> c_int;
}

fn main() {
    let c_str = b"hello ffi\0".as_ptr() as *const c_char;
    unsafe {
        puts(c_str); // 外部函数调用必须unsafe
    }
}
```

### 2. 链接第三方 C 库
两种方式：
1. **系统动态库**：`#[link(name = "m")]` 链接 libm、libssl 等
```rust
#[link(name = "m")] // 链接数学库 libm.so / libm.dylib
extern "C" {
    fn sin(x: f64) -> f64;
}
```
2. **静态编译自定义 C 文件**：搭配 `cc` build.rs
```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .compile("libfoo.a");
}
```

## 三、场景2：Rust 导出库给 C 调用（最常用）
把 Rust 编译成 `.so`(Linux) / `.dll`(Windows) / `.dylib`(macOS)，C/C++ 直接调用。

### 1. 基础导出函数写法
```rust
// lib.rs
use libc::c_int;

// 导出给C调用
#[no_mangle] // 禁止Rust名字混淆，导出原始函数名
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}
```
- `#[no_mangle]`：关键！Rust 默认会混淆函数名，不加 C 找不到符号；
- `extern "C"`：使用 C 调用约定；
- `pub`：确保函数对外可见。

### 2. Cargo 配置输出动态库
`Cargo.toml`
```toml
[lib]
crate-type = ["cdylib"] # 生成C兼容动态库
# staticlib 生成静态库 .a
# dylib Rust专用动态库，不要用于FFI
```
编译：
```bash
cargo build --release
# target/release/libxxx.so (linux)
# target/release/xxx.dll (windows)
# target/release/libxxx.dylib (mac)
```

### 3. C 侧调用示例
```c
// main.c
#include <stdio.h>

int add(int a, int b);

int main() {
    printf("%d\n", add(1,2));
    return 0;
}
```
编译链接：
```bash
gcc main.c -L./target/release -lxxx -o main
LD_LIBRARY_PATH=./target/release ./main # Linux运行
```

## 四、字符串传递（高频踩坑点）
### 1. Rust → C：生成 C 零结尾字符串
```rust
use libc::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn print_msg() -> *mut c_char {
    // CString 自动追加 \0
    let cstr = CString::new("rust msg").unwrap();
    cstr.into_raw() // 交出所有权，Rust不再管理内存
}

// C必须调用这个释放，否则内存泄漏
#[no_mangle]
pub extern "C" fn free_msg(s: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(s); // 回收内存，自动drop
    }
}
```
规则：**谁分配谁释放**，不能 Rust 分配、C free，会崩溃。

### 2. C → Rust：读取 C 字符串
```rust
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn take_cstr(s: *const c_char) {
    unsafe {
        if s.is_null() { return; }
        let c_str = CStr::from_ptr(s);
        let rust_str = c_str.to_str().unwrap(); // &str
        println!("got str: {}", rust_str);
    }
}
```
- `CStr`：借用外部 C 字符串，不拥有内存；
- 不能直接转 `String` 否则重复释放。

## 五、结构体跨语言传递
### 1. 保证内存布局一致
- C 兼容结构体必须加 `#[repr(C)]`，按 C 规则排列字段；
- 不加 `#[repr(C)]` Rust 会重排字段，C 读取错乱。
```rust
#[repr(C)]
#[derive(Debug)]
pub struct Point {
    x: libc::c_int,
    y: libc::c_int,
}

#[no_mangle]
pub extern "C" fn new_point(x: c_int, y: c_int) -> Point {
    Point { x, y }
}
```
C 侧对应结构体：
```c
typedef struct {
    int x;
    int y;
} Point;
```

### 2. 不透明类型（推荐，封装）
不想暴露结构体内部，用**裸指针不透明句柄**，C 只能传指针不能访问字段：
```rust
#[repr(C)]
pub struct Context {
    // 内部复杂Rust结构，不暴露给C
    data: Vec<u8>,
}

// 只返回指针，C看不到内部
#[no_mangle]
pub extern "C" fn ctx_new() -> *mut Context {
    let ctx = Box::new(Context { data: Vec::new() });
    Box::into_raw(ctx)
}

#[no_mangle]
pub extern "C" fn ctx_free(p: *mut Context) {
    if !p.is_null() {
        unsafe { Box::from_raw(p); }
    }
}
```

## 六、内存安全核心坑（FFI 最容易出错）
1. **所有权混乱**
   - `into_raw()`：Rust 放弃所有权，必须手动通过 `from_raw` 回收；
   - 禁止跨语言释放：Rust 堆由 Rust allocator 管理，C `free()` 无法释放。
2. **空指针、野指针**
   所有裸指针操作前判空 `ptr.is_null()`。
3. **生命周期失效**
   局部变量转裸指针返回外部，函数结束内存被释放，悬垂指针。
4. **字符串无终止符**
   普通 `&[u8]` 没有 `\0`，C 读取会越界。
5. **结构体布局不匹配**
   忘记 `#[repr(C)]` 导致字段偏移错乱。
6. **panic 跨语言**
   Rust 函数 panic 会直接终止整个进程，不会回传给 C；
   对外 FFI 函数尽量捕获 panic：`std::panic::catch_unwind`。

## 七、常用辅助库简化 FFI
1. **libc**：平台无关 C 基础类型、常量、函数声明
   ```toml
   [dependencies]
   libc = "0.2"
   ```
2. **cc**：build.rs 编译内嵌 C 代码
3. **bindgen**：自动从 C 头文件生成 Rust FFI 绑定（大型C库必备）
   - 读取 `.h`，自动生成 `extern "C"`、`#[repr(C)]` 结构体
4. **cbindgen**：反向，从 Rust 代码自动生成 C `.h` 头文件
   开发 Rust 库给 C 调用首选，不用手写头文件。

## 八、典型工作流
### 流程A：Rust 使用现有 C 库
1. bindgen 解析 C header 生成 Rust 绑定；
2. `#[link]` 或 build.rs 链接库；
3. unsafe 块调用 C 函数，处理字符串/指针。

### 流程B：Rust 封装逻辑，提供给 C/C++/Python/Java 调用
1. 写 `#[no_mangle] extern "C"` 导出接口；
2. `#[repr(C)]` 定义互通结构体；
3. cbindgen 自动生成 `.h`；
4. 编译 `cdylib` 动态库；
5. 上层语言通过 C ABI 调用。

## 九、扩展：其他语言调用 Rust FFI 库
- **Python**：ctypes / cffi 加载 `.so/.dll`
- **Go**：cgo 调用 Rust cdylib
- **Java**：JNI 中间层 + Rust FFI
- **Node.js**：node-ffi/napi

所有语言统一走 C ABI，Rust 无需单独适配各语言。

## 十、最佳实践总结
1. 所有跨语言函数：`#[no_mangle] + extern "C"`；
2. 互通结构体强制 `#[repr(C)]`；
3. 字符串一律使用 `CString` / `CStr`；
4. 内存遵循「谁分配、谁释放」，提供专门释放函数；
5. 对外接口尽量用不透明指针隐藏 Rust 复杂类型；
6. FFI 入口加 panic catch_unwind，防止进程崩溃；
7. 使用 bindgen / cbindgen 减少手写绑定出错；
8. 所有裸指针操作必须在 `unsafe`，并做空指针检查。

需要我给你一份可直接运行的最小完整示例（Rust 导出动态库 + C 调用 + 字符串传递 + 结构体）吗？