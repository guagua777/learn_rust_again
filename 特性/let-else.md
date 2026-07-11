# Rust `let-else` 完整讲解
## 一、是什么
`let-else` 是 Rust 1.65 引入的语法，专门简化**匹配失败直接提前返回/跳出**的场景：
- 成功：绑定变量，继续走后面代码
- 失败：执行 `else` 块，`else` 块**必须发散（diverge）**，也就是必须 `return` / `panic!` / `break` / `continue`，不能正常走到代码末尾

### 解决的痛点
传统 match / if let 多层缩进、大量嵌套，`let-else` 扁平化代码。

## 二、基础语法
```rust
let 模式 = 表达式 else {
    // 匹配失败，必须发散
    return;
};
// 匹配成功，这里可以使用绑定的变量
```

## 三、常用示例
### 1. Option 取值（最常用）
传统 if let 写法（缩进变深）
```rust
fn get_name(opt: Option<&str>) {
    if let Some(name) = opt {
        println!("{}", name);
        // 一堆业务逻辑
    } else {
        eprintln!("无名字");
        return;
    }
}
```

let-else 扁平化：
```rust
fn get_name(opt: Option<&str>) {
    let Some(name) = opt else {
        eprintln!("无名字");
        return;
    };
    // 直接使用 name，无嵌套
    println!("{}", name);
}
```

### 2. Result 快速失败
```rust
fn read_num() -> Result<i32, &'static str> {
    let res = Ok(42);
    let Ok(n) = res else {
        return Err("读取失败");
    };
    Ok(n * 2)
}
```

> 补充：Result 日常更多用 `?`，但多层复杂模式匹配时 let-else 更灵活。

### 3. 解构匹配
结构体、元组都支持模式解构：
```rust
struct User {
    id: u32,
    age: Option<u8>,
}

fn check_user(u: User) {
    let User { id, age: Some(age) } = u else {
        panic!("用户年龄不存在");
    };
    println!("id:{}, age:{}", id, age);
}
```

元组示例：
```rust
let (_, Some(val)) = (10, Some(99)) else {
    return;
};
println!("{}", val);
```

### 4. 搭配循环 + break（结合上一节 label）
```rust
'outer: loop {
    let Some(x) = get_opt() else {
        break 'outer; // else 发散：跳出外层循环
    };
    println!("{}", x);
}
```

## 四、关键规则（硬性约束）
1. **else 块必须发散，不能正常结束**
   下面代码编译报错：
   ```rust
   // 错误！else 走完没有 return/panic/break
   let Some(v) = None else {
       println!("none");
   };
   ```
   合法发散操作：
   - `return` 函数返回
   - `panic!` / `todo!` / `unreachable!` 崩溃
   - `break` / `continue` 跳出循环

2. 变量作用域
   `let-else` 绑定的变量，仅在 `else` 块**之后**可见，`else` 内部访问不到。
   ```rust
   let Some(x) = opt else {
       println!("{}", x); // 编译错误：x 还未绑定
       return;
   };
   ```

3. 只支持**单分支匹配**
   `let-else` 只能写一种成功模式，不能像 match 处理多种分支；多分支还是要用 `match`。

## 五、和 if let / match 对比
| 写法       | 适用场景                     | 代码结构       |
|------------|------------------------------|----------------|
| `if let`   | 匹配成功后有大量逻辑         | 嵌套缩进       |
| `match`    | 多种分支需要分别处理         | 完整分支匹配   |
| `let-else` | 匹配失败直接终止当前流程     | 扁平化、无嵌套 |

## 六、实战场景总结
1. 函数开头快速校验 Option/Result，失败直接 return；
2. 解构结构体、元组，缺少关键字段直接 panic；
3. 循环内取值，取不到就 break 终止循环；
4. 消除多层 if let 嵌套，提升可读性。