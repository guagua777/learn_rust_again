


          

        
            
toolName: view_files
            
status: failed
          
            
filePath: /home/guagua/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs
          
`Box::into_raw` 是 `Box<T>` 的一个核心方法，用于**将堆上的所有权转换为裸指针**。

## 函数签名

```rust
pub fn into_raw(b: Self) -> *mut T
```

## 它做了什么

1. **消费（consume）** `Box<T>`，取得堆上数据的所有权
2. 返回一个 `*mut T` 裸指针，指向堆上原来的数据
3. **不会调用析构函数**，不会释放内存——数据的生命周期现在由你手动管理

## 关键点

```rust
let b = Box::new(42);     // 在堆上分配，值为 42
let ptr = Box::into_raw(b); // b 被消费，得到 *mut i32

// 此时：
// - b 已经被 move，不能再使用
// - ptr 指向堆上的 42
// - 内存没有被释放！你需要手动管理
```

## 对应的回收方法：`Box::from_raw`

```rust
unsafe {
    let b = Box::from_raw(ptr); // 从裸指针重建 Box，恢复所有权
} // b 离开作用域时，堆内存被正确释放
```

## 典型使用场景

**1. FFI 交互**——把 Rust 的堆数据传给 C：

```rust
let b = Box::new(MyStruct { x: 1, y: 2 });
let ptr = Box::into_raw(b);
// 把 ptr 传给 C 函数，C 端负责或稍后 Rust 端用 from_raw 回收
```

**2. 构建自引用/链式结构**：

```rust
struct Node {
    val: i32,
    next: *mut Node,  // 裸指针，不受借用检查约束
}
```

## 与 `as_ref()` / `as_ptr()` 的区别

| 方法 | 所有权 | 内存释放 |
|------|--------|----------|
| `Box::as_ptr(&self)` | 借用，Box 仍存在 | Box 析构时正常释放 |
| `Box::into_raw(self)` | 转移，Box 被消费 | **不会释放**，需手动 `from_raw` 回收 |

## ⚠️ 安全提醒

- 必须保证每个 `into_raw` 都有对应的 `from_raw`，否则**内存泄漏**
- `from_raw` 必须在**同一个分配器**上调用
- 重复 `from_raw` 同一个指针会导致 **double free**
        