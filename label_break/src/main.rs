static S: String = String::new();

fn main() {
    println!("Hello, world!");

    // let s: str = "hello";

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

    let res = 'search: loop {
        for n in 1..10 {
            if n == 5 {
                // 跳出标签循环并返回数值
                break 'search n * 2;
            }
        }
    };
    println!("{}", res); // 输出 10

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

    let mut x = 10;
    let r1 = &mut x; // 第一层可变借用 &mut i32
    let r2 = &*r1; // Reborrow：再借用，生成 &i32
                   // r2 只读，r1 暂时被冻结
    println!("{} {}", r1, r2);

    *r1 = 20;
    println!("{}", r1);
}
