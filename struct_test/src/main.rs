#[derive(Debug)]
#[derive(Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}


// 为什么需要 #[derive(Clone, Copy)] 注解
// 问题分析
// 让我分析一下代码中的关键部分，解释为什么需要 #[derive(Clone, Copy)] 注解：

// 1. max 方法的定义

// rust
// fn max(self, other: Self) -> Self {
//     let w = self.width.max(other.width);
//     let h = self.height.max(other.height);
//     Self {
//         width: w,
//         height: h,
//     }
// }
// main.rs

// 注意 max 方法的第一个参数是 self，而不是 &self。这意味着：

// 调用 max 方法时，会获取调用者的所有权
// 方法执行完毕后，原始对象会被消耗掉
// 2. set_to_max 方法的定义

// rust
// fn set_to_max(&mut self, other: Rectangle) {
//     *self = self.max(other);
// }
// main.rs

// 这里的问题是：

// set_to_max 方法接收的是 &mut self（可变引用）
// 但是当它调用 self.max(other) 时，max 方法需要获取 self 的所有权
// 然而，我们只有 self 的引用，没有所有权可以转移
// 为什么 #[derive(Clone, Copy)] 可以解决问题
// 当我们为 Rectangle 结构体添加 #[derive(Clone, Copy)] 注解时：

// Copy trait 允许 Rust 在需要时自动复制值，而不是移动它
// Clone trait 是 Copy trait 的前提，提供了显式复制的能力
// 在这种情况下：

// 当调用 self.max(other) 时，由于 Rectangle 实现了 Copy trait
// Rust 会自动创建 self 的一个副本
// max 方法获取的是这个副本的所有权，而不是原始值的所有权
// 原始的 self（通过 &mut self 引用的）保持不变
// 当 max 方法返回新的 Rectangle 实例时，我们可以将其赋值给 *self
// 技术细节
// 所有权与借用：Rust 的核心特性之一是所有权系统，它确保内存安全
// 移动语义：默认情况下，当你将一个值传递给函数时，会发生所有权转移（移动）
// 复制语义：实现了 Copy trait 的类型在传递时会被自动复制，而不是移动
// Copy trait 的要求：
// 类型必须实现 Clone trait
// 类型不能包含实现了 Drop trait 的字段
// 类型必须是简单的，通常是基本类型或由基本类型组成的结构体
// 代码验证
// 如果我们移除 #[derive(Clone, Copy)] 注解，编译会失败，错误信息会类似于：


// plainText
// error[E0507]: cannot move out of `*self` which is behind a mutable reference
//   --> src/main.rs:30:18
//    |
// 30 |         *self = self.max(other);
//    |                  ^^^^ move occurs because `*self` has type `Rectangle`, which does not implement the `Copy` trait
// 这明确告诉我们，由于 Rectangle 没有实现 Copy trait，我们不能从可变引用后面移动值。

// 总结
// #[derive(Clone, Copy)] 注解使得 Rectangle 类型具有了复制语义，允许 Rust 在需要时自动创建值的副本，而不是移动原始值。这使得我们可以在持有可变引用的情况下调用需要所有权的方法，因为 Rust 会自动复制出一个临时值传递给该方法。

// 就像对待i32一样，把值复制一份


impl Rectangle {
    // &self 为当前实例的引用，等同于 self: &Self
    // 也可以为self，表示方法会获取实例的所有权
    // 也可以为&mut self，表示方法会获取实例的可变引用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Self {
            width: w,
            height: h,
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}


impl Rectangle {
    // 返回类型为Self，表示返回当前类型的实例
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}