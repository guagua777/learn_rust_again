use std::string;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // test1();
    test2();
    test3();
}

// fn test1() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     let mut sort_operations = vec![];
//     let value = String::from("closure called");

//     list.sort_by_key(|r| {
//         // 报错：cannot move out of `value`, a captured variable in an `FnMut` closure
//         // 捕获了环境中的 value 变量，且移动了 value 变量，所以只能调用一次，FnOnce trait
//         // 所以此处会报错
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{list:#?}");
// }



fn test2() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}



// FnOnce、FnMut、Fn仅代表捕获的内容，不代表闭包的返回值
fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {   
    move || {
        s_ref.to_string()
    }
}

// fn make_a_cloner123(s_ref: &str) -> (impl Fn()) -> String {   
//     move || {
//         s_ref.to_string()
//     }
// }

fn make_a_cloner12(s_ref: &str) -> impl (Fn() -> String) {   
    move || {
        s_ref.to_string()
    }
}

// 生命周期
// 1. 引用：&'a str
// 2. 值：String + 'a

// 这块不是太懂
fn make_a_cloner2<'a>(s_ref: &'a str) -> (impl Fn() -> String + 'a) {   
    move || {
        s_ref.to_string()
    }
}


// 右边这个文件中的

// fn make_a_cloner2<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {   

//     move || {

//         s_ref.to_string()

//     }

// }

// 里面的impl，是实现的Fn()，还是Fn() -> String ？​
// TRAE AI
// Thought process
// 用户问的是关于 Rust 代码中的 impl Fn() -> String + 'a 这个返回类型的问题，具体是想知道 impl 是实现的 Fn() 还是 Fn() -> String。

// 首先，我需要理解 Rust 中的 trait 语法。在 Rust 中，Fn() -> String 是一个完整的 trait 类型，表示一个无参数、返回 String 的闭包。而 impl Fn() -> String + 'a 则表示返回一个实现了 Fn() -> String trait 并且生命周期为 'a 的类型。

// 让我先查看一下文件的内容，确认上下文，然后详细解释这个语法。

// 让我先查看文件内容，了解完整的代码上下文，然后解释 impl Fn() -> String + 'a 的含义。

// closure_test/src/main.rs
// 在 fn make_a_cloner2<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a 中，impl 实现的是 Fn() -> String 这个 trait。

// 详细解释：
// Fn() -> String 是什么？

// 这是一个完整的 trait 定义，表示"一个不接受参数（()）并返回 String 类型的闭包"。
// Fn() 只是这个 trait 的一部分，完整的签名包括返回类型 -> String。
// impl 关键字的作用

// impl 在这里表示"返回一个实现了特定 trait 的类型"。
// 具体来说，它返回的是一个闭包，这个闭包实现了 Fn() -> String trait。
// + 'a 的含义

// 这是生命周期约束，表示返回的闭包的生命周期至少与 'a 一样长。
// 因为闭包通过 move 关键字捕获了 s_ref（一个生命周期为 'a 的引用），所以闭包本身的生命周期不能超过 s_ref 的生命周期。
// 与 make_a_cloner 的对比

// make_a_cloner 函数没有显式指定生命周期，Rust 会自动推断。
// make_a_cloner2 显式指定了生命周期 'a，使代码意图更清晰。
// 技术要点：
// 闭包 trait 层次：Rust 中闭包有三个 trait：FnOnce（只能调用一次）、FnMut（可修改捕获的变量）、Fn（不可修改捕获的变量）。
// 生命周期约束：当闭包捕获引用时，必须确保闭包的生命周期不超过被捕获引用的生命周期。
// impl Trait 语法：用于返回实现了特定 trait 的类型，而不需要具体指定类型名称。
// 所以，impl 实现的是完整的 Fn() -> String trait，而不仅仅是 Fn()。




fn test3() {
    let s = String::from("hello");
    let cloner = make_a_cloner(&s);
    // drop(s);
    println!("{}", cloner());
}


fn cousure_define() {
    let _a = |x: i32| x * 2;
}

// js 中
// let identity = x => x;
// 其中的 x => x 就是一个匿名函数，或者说是一个闭包（closure）。这个闭包接受一个参数 x，并返回它自己。
// 闭包可以捕获外部作用域中的变量，这使得它们非常灵活和强大。
// 在 Rust 中，闭包也是一种匿名函数，可以捕获环境中的变量，并且可以根据需要进行不同的捕获方式（按值、按引用等）。


// Fn() -> String，这才是一个完整的闭包，有输入有输出，输入是Fn()，输出是String
// Fn()，只是一个闭包的输入部分，没有输出，所以不是一个完整的闭包