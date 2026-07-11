# Rust `break 'label` 带标签break 完整讲解
## 1. 作用
Rust 支持给循环打**标签（label）**，搭配 `break '标签名` 可以**直接跳出多层嵌套循环**，不用额外标记变量层层退出。
普通无标签 `break` 只能跳出**最内层**循环；带标签 `break` 能指定跳出哪一层循环。

### 语法规则
1. 标签格式：`'标识符:`，必须以单引号 `'` 开头，结尾加冒号
2. 循环只能是 `loop` / `for` / `while`
3. 跳出：`break '标签;`
4. 同时支持带返回值：`break '标签 表达式;`（仅 `loop` 支持返回值）

## 2. 基础示例：跳出双层循环
```rust
// 给外层循环打标签 outer
'outer: for i in 1..=3 {
    for j in 1..=3 {
        if i == 2 && j == 2 {
            // 直接跳出外层 'outer 循环，两层一起退出
            break 'outer;
        }
        println!("i={}, j={}", i, j);
    }
}
```
输出：
```
i=1, j=1
i=1, j=2
i=1, j=3
i=2, j=1
```
逻辑：当 `i=2,j=2` 触发 `break 'outer`，直接终止外层for循环，程序不再继续。

对比普通无标签break：只会跳出内层j循环，外层i还会继续执行。

## 3. loop 带标签 + break 返回值
`loop` 是唯一能通过 `break` 返回值的循环，标签也兼容：
```rust
let res = 'search: loop {
    for n in 1..10 {
        if n == 5 {
            // 跳出标签循环并返回数值
            break 'search n * 2;
        }
    }
};
println!("{}", res); // 输出 10
```

## 4. 搭配 continue 'label
补充：标签同样支持 `continue 'label`，作用是**跳到指定循环的下一轮**
```rust
'outer: for i in 1..=3 {
    println!("外层i:{}", i);
    for j in 1..=3 {
        if j == 2 {
            // 直接跳到外层循环下一次迭代
            continue 'outer;
        }
        println!("内层j:{}", j);
    }
}
```
输出：
```
外层i:1
内层j:1
外层i:2
内层j:1
外层i:3
内层j:1
```
j=2时，不会打印内层剩余内容，直接外层i+1。

## 5. 使用限制
1. 标签**只能作用于循环**（loop/for/while），不能给if、match、代码块打标签用break跳出；
2. `break 'label` 必须在该标签循环的**内部嵌套层级**，不能跨函数、跨代码块；
3. 标签命名遵循普通标识符规则，不能是关键字；
4. 一个循环只能绑定一个标签，同一作用域标签不能重名。

## 6. 常见场景
- 多层嵌套遍历数组、矩阵，找到目标数据后直接全部退出；
- 多层循环搜索匹配，避免用布尔标记（如 `let mut found = false`）层层判断；
- 简化深层循环退出逻辑，代码更干净。

## 7. 不用标签的替代写法（对比）
不用标签时需要手动标记判断，代码冗余：
```rust
let mut found = false;
for i in 1..=3 {
    for j in 1..=3 {
        if i == 2 && j == 2 {
            found = true;
            break;
        }
    }
    if found {
        break;
    }
}
```
带标签 `break 'outer` 省去额外布尔变量，可读性更高。