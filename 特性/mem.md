# Rust 三个内存操作函数：std::mem::take / replace / swap
三者都属于 `std::mem` 模块，核心用途是**转移/替换变量的值、处理所有权、避免重复析构，方便原地修改数据**

## 1. std::mem::replace
```rust
pub fn replace<T>(dest: &mut T, value: T) -> T
```
- **作用**：把 `dest` 可变引用指向的值，**替换成新传入的 value，返回原来的旧值**
- 本质：取出旧值，写入新值
- 示例
```rust
use std::mem;

let mut x = String::from("旧值");
let old = mem::replace(&mut x, String::from("新值"));
println!("旧值:{}", old); // 旧值:旧值
println!("x:{}", x);     // x:新值
```
- 适用场景：
  - 需要保留原来的值，同时原地更新变量
  - 结构体字段替换、取出原有数据

## 2. std::mem::take
```rust
pub fn take<T: Default>(dest: &mut T) -> T
```
- **作用**：取出 `dest` 原来的值，**把 dest 重置为 T 的默认值（Default），返回原来的值**
- 等价写法：`mem::replace(dest, T::default())`
- 要求类型必须实现 `Default` trait
- 示例
```rust
use std::mem;

let mut s = String::from("hello");
let old = mem::take(&mut s);
println!("old:{}", old); // old:hello
println!("s:{}", s);     // s: (空字符串，String默认值)
```
- 适用场景：
  - 清空变量、转移原有数据，变量恢复空/初始状态
  - 比如清空集合、临时取出数据，原变量继续可用
- 和 replace 的区别：take 不用手动传新值，直接用默认值

## 3. std::mem::swap
```rust
pub fn swap<T>(a: &mut T, b: &mut T)
```
- **作用**：**直接交换两个可变引用变量的值，无返回值**
- 不是简单赋值，是高效原地交换内存内容，不产生额外所有权拷贝
- 示例
```rust
use std::mem;

let mut a = 10;
let mut b = 20;
mem::swap(&mut a, &mut b);
println!("a={}, b={}", a, b); // a=20, b=10
```
- 变体：还有 `swap(&mut self, other: &mut Self)` 方法（很多容器自带）
- 适用场景：两个变量互相交换内容、排序/双变量交换逻辑

## ✅ 三者对比总结
| 函数 | 签名核心 | 核心行为 | 约束 | 返回值 |
|---|---|---|---|---|
| `mem::replace` | `replace(&mut T, new:T) -> T` | 自定义新值替换，取出旧值 | 无额外 trait | 原值 |
| `mem::take` | `take(&mut T) -> T` | 默认值替换，取出旧值 | `T: Default` | 原值 |
| `mem::swap` | `swap(&mut T, &mut T)` | 互相交换两个变量 | 无额外 trait | 无返回值 |

### 一句话记忆
- `replace`：我给新值，换回旧值
- `take`：自动置默认值，拿走旧值
- `swap`：两个变量直接互换

### 底层共性
都是操作栈/堆内存所有权转移，**避免手动赋值带来的 clone、重复 drop 问题，是 Rust 安全高效原地修改数据的惯用写法**