use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    // 栈上的对象发生了移动
    // 业务场景上，不能移动a的所有权的时候u，就要使用clone
    let _b = a; // move，不是clone
    println!("{}", a); // ❌编译报错，a已经被move走了
}

// 生命周期结束了，依附于该生命周期的所有东西都失效了
