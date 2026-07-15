use std::thread;






          
// 这是一个很好的 Rust 语法问题！

// ## `FnOnce() -> T + Send + 'scope` 是一个**复合 trait bound（复合 trait 约束）**

// 整个表达式用 `+` 连接了三个约束，要求类型 `F` **同时满足**以下所有条件：

// | 部分 | 含义 |
// |------|------|
// | `FnOnce() -> T` | 实现了 `FnOnce` trait，且调用签名是「无参数，返回 `T`」 |
// | `Send` | 实现了 `Send` trait，可以安全地跨线程移动 |
// | `'scope` | 满足生命周期约束 `'scope`，即至少活到 `'scope` 结束 |

// ### 逐步拆解

// **1. `FnOnce() -> T`** — 这是 Rust 的**函数 trait 语法糖**

// `FnOnce` 本身是一个 trait，但 Rust 允许用 `FnOnce(Args) -> Return` 的写法来同时指定参数和返回值类型。底层等价于：

// ```rust
// FnOnce<Args = (), Output = T>
// ```

// 所以 `FnOnce() -> T` 表示「可以被调用一次、不接受参数、返回 `T` 的闭包/函数」。

// **2. `+ Send`** — 叠加 `Send` 约束

// `+` 在 trait bound 中的作用是"且"（AND），表示类型还必须实现 `Send`，即可以安全地在线程间转移所有权。

// **3. `+ 'scope`** — 叠加生命周期约束

// `+` 也可以连接生命周期约束，表示类型的引用必须至少在 `'scope` 期间有效。在 `scoped` 线程的上下文中，这确保闭包不会比作用域活得更久。

// ### 等价的 where 子句写法

// ```rust
// F: FnOnce() -> T + Send + 'scope

// // 等价于：
// where
//     F: FnOnce() -> T,
//     F: Send,
//     F: 'scope,
// ```

// ### 在 `scoped.rs` 中的意义

// 在 `scope::spawn` 的上下文中，这个约束确保：

// - 闭包**可以被调用**（`FnOnce`）
// - 闭包**可以安全地发送到子线程**（`Send`）
// - 闭包**不会比作用域 `'scope` 活得更久**（`'scope`），从而保证不会出现悬垂引用

// 这正是 Rust scoped threads 安全性的核心——通过 `'scope` 生命周期，编译器在编译期就保证了子线程不会逃逸出作用域，从而避免了 `std::thread::spawn` 中 `'static` 的需求。
        

fn main() {
    let data = vec![1, 2, 3, 4];

    // 参数为Scope的引用
    thread::scope(|s| {
        // 直接借用外部 data，不需要 move、不需要 'static
        let t1 = s.spawn(|| {
            println!("{:?}", &data[0..2]);
        });
        let t2 = s.spawn(|| {
            println!("{:?}", &data[2..]);
        });

        // 可主动等待，也可让 scope 自动等待全部线程结束
        t1.join().unwrap();
        t2.join().unwrap();
    });
    // 离开 scope 时：所有 scope 内线程必定已结束，data 可正常继续使用
    println!("主线程结束: {:?}", data);
}