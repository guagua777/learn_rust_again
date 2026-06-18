1. 移动所有权
2. clone
3. 引用 与 借用
4. 解引用
5. 



Rust move 后变量失效完整原理
一、核心前提：所有权（Ownership）规则
Rust 每个堆分配数据只有一个所有者：
栈上存指针 / 长度 / 容量（胖指针），堆存真实数据；
赋值、传参、返回值都会触发 move（所有权转移）；
所有权转移后，原变量立刻失效（uninitialized），再使用编译报错。
二、move 底层发生了什么（内存视角）
示例代码
rust
运行
let s1 = String::from("hello"); // s1 是栈上结构体，持有堆字符串所有权
let s2 = s1; // move，所有权从 s1 → s2
println!("{}", s1); // 编译错误：s1 已被移动，不可使用
1. String 栈结构定义
rust
运行
struct String {
    ptr: *mut u8,   // 堆数据指针
    len: usize,      // 有效长度
    cap: usize,      // 分配容量
}
s1 存放在栈帧，指向堆上 "hello" 字节数组。
2. let s2 = s1; move 过程
栈拷贝：把 s1 的 ptr/len/cap 三个字段按位复制到栈上新变量 s2；
编译器标记 s1 为 “已移动（moved）”：
Rust 不会清空、不会置空 s1 的栈内存，只是编译器静态标记该变量不再合法；
堆数据不会复制（这是 move 和 clone 的核心区别），堆所有权现在仅属于 s2。
关键点：
物理内存上 s1 的栈数据还在，指针依旧指向堆；
但 Rust 静态检查器（borrow checker）记录：s1 所有权已移交，属于未初始化状态。
三、为什么不能再用 s1？两种层面解释
1. 编译静态层面（最关键）
Rust 的移动语义是编译期检查，不运行时判断：
编译器维护每个栈变量的初始化状态；
当变量发生 move 后，状态从 Initialized → Moved；
任何读取、借用、传参、打印 Moved 状态变量，直接报 E0382 错误：
plaintext
error[E0382]: use of moved value: `s1`
编译器不需要运行，单纯数据流分析就能识别：
数据流：s1 → 赋值给 s2 → 后续无重新赋值 → 后续使用非法。
2. 运行时内存层面（无任何修改）
move 不会修改栈、堆内存：
s1 的栈结构体字节完全保留，指针依然有效；
堆内存不会释放，也不会复制；
没有清零、没有置 null、没有任何运行时指令处理失效逻辑。
失效纯粹是编译期规则限制，不是运行时内存损坏。
四、什么时候变量会恢复有效？
只有重新给被 move 的变量绑定新值，状态重置为 Initialized：
rust
运行
let s1 = String::from("a");
let s2 = s1; // s1 标记为 moved
// println!("{}", s1); // 报错
let s1 = String::from("b"); // 重新绑定，s1 恢复可用
println!("{}", s1); // 正常
原理：新的 let s1 遮蔽旧变量，分配全新初始化状态。
五、区分 Copy 类型与 Move 类型
1. Copy 类型（栈小数据，不会 move）
实现 Copy trait：i32、bool、f64、char、固定大小数组、引用 &T
赋值时直接复制副本，原变量保留所有权，不会失效：
rust
运行
let a = 10;
let b = a; // copy，不是move
println!("{}", a); // 合法
编译器逻辑：Copy 类型赋值后原变量仍保持 Initialized。
2. 非 Copy 类型（堆数据，必触发 move）
String、Vec、Box、自定义无 Copy 结构体，赋值一律 move，原变量失效。
六、函数传参中的 move 失效演示
rust
运行
fn take_own(s: String) {
    // s 获得所有权
}

fn main() {
    let x = vec![1,2,3];
    take_own(x); // move：所有权转移到函数参数s
    println!("{:?}", x); // E0382，x已移动失效
}
执行流程：
调用函数时，栈上 x 的 Vec 结构体拷贝给参数 s；
编译器标记 x 为 moved；
函数结束，参数 s 离开作用域，自动 drop，释放堆内存；
此时 x 哪怕栈内存还在，也绝对禁止访问，防止双重释放（double free）。
七、底层设计目的：防止 double free
如果 move 后允许原变量继续使用：
s1、s2 都认为自己拥有堆数据；
作用域结束时，两个变量都会执行 drop，释放同一块堆内存；
触发未定义行为（内存损坏、崩溃）。
Rust 通过move 后原变量静态失效，从编译期杜绝双重释放问题。