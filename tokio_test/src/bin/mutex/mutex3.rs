use tokio::time::sleep;
// use std::sync::Mutex;
use tokio::sync::Mutex;
use std::time::Duration;
use std::sync::LazyLock;


// OnceCell和LazyCell呢？
// 先一句话分清：
// OnceCell<T>：先有空位，手动 / 按需填值一次，可以中途检查是否初始化、手动 set、多次 get_or_init。
// LazyCell<T, F>：创建时就绑定好初始化函数 F，第一次 *deref（&lazy） 自动运行 F 填值，之后只读；更 “懒”、更封装。
// 下面从本质、用法、场景、能不能互相替代来讲清楚。
// 一、本质区别（核心）
// OnceCell<T>
// rust
// 运行
// pub struct OnceCell<T> { ... }
// 状态：空 → 被写一次 → 只读
// 创建：const fn new()，不带初始化逻辑
// 初始化时机：
// 手动 set(value)
// 或 get_or_init(|| value)
// 特点：你控制何时初始化；可以先判断是否初始化，再决定做什么
// LazyCell<T, F>
// rust
// 运行
// pub struct LazyCell<T, F = fn() -> T> { ... }
// 状态：持有 F → 第一次解引用运行 F → 存结果、只读
// 创建：LazyCell::new(|| ...)，构造时就把初始化函数存进去
// 初始化时机：第一次访问（deref）自动触发
// 特点：完全自动懒加载；你不直接调用初始化，用的时候才跑
// 二、用法对比（单线程）
// 1）OnceCell：手动控制流程
// rust
// 运行
// use std::cell::OnceCell;

// let cell = OnceCell::new();
// assert!(cell.get().is_none());

// // 手动初始化
// cell.set("hello".to_string()).unwrap();

// // 或用 get_or_init（没初始化才跑闭包）
// let v = cell.get_or_init(|| "world".to_string());

// // 拿引用，不复制
// println!("{}", v);
// 可先 get() 判断，再决定是否初始化
// 初始化逻辑可以分散、动态决定
// 2）LazyCell：创建即绑定，访问即初始化
// rust
// 运行
// use std::cell::LazyCell;

// // 构造时就给好“怎么创建值”
// let lazy = LazyCell::new(|| {
//     println!("init");
//     "hello".to_string()
// });

// println!("before access");
// // 第一次解引用才执行闭包
// let s: &String = &*lazy;
// println!("{}", s);
// 输出：
// plaintext
// before access
// init
// hello
// 初始化逻辑固定在构造时，不能中途换
// 用起来像 “直接是 &String”，自动懒加载
// 三、关键差异点（表格）
// 表格
// 特性	OnceCell<T>	LazyCell<T, F>
// 初始化逻辑	不自带，外部提供	创建时绑定 F
// 触发时机	手动 set /get_or_init	第一次 deref（&*x）
// 能否检查未初始化	能（get () -> Option）	不能，访问即初始化
// 能否延迟决定初始化方式	能	不能，F 固定
// 外观	像 “可空、一次赋值的容器”	像 “直接是 T，懒创建”
// 多线程对应	OnceLock<T>	LazyLock<T, F>
// 四、使用场景怎么选？
// 选 OnceCell
// ✅ 需要先判断是否初始化，再决定行为
// ✅ 初始化逻辑复杂、动态、依赖运行时条件
// ✅ 想手动控制初始化时机（比如某个事件后）
// ✅ 同一容器可能有多种初始化路径
// 典型：
// 局部单例、延迟加载但要手动触发
// 有条件的初始化（某些分支才需要创建）
// 选 LazyCell
// ✅ 只想用的时候自动创建，不想写 if None 逻辑
// ✅ 初始化逻辑简单、无依赖、固定
// ✅ 想代码更简洁，直接当 &T 用
// 典型：
// 昂贵计算 / 大对象，不一定会用到
// 函数内局部懒变量，每次调用最多初始化一次
// 五、能不能只用其中一个？
// 1）只用 OnceCell 模拟 LazyCell
// 可以，但丑：
// rust
// 运行
// let cell = OnceCell::new();
// let v = cell.get_or_init(|| { /* ... */ });
// 每次要写 get_or_init，不如 &*lazy 自然。
// 2）只用 LazyCell 模拟 OnceCell
// 做不到：
// LazyCell 无法先检查是否初始化而不触发初始化
// 初始化函数不能动态替换，构造时就定死了
// 所以：
// OnceCell = 可控的一次初始化
// LazyCell = 自动的一次懒初始化
// 两者是互补，不是谁能完全替代谁
// 六、顺带一提：线程安全版
// 单线程：OnceCell / LazyCell
// 多线程：OnceLock / LazyLock（std::sync 下）



// static DATA: Mutex<u32> = Mutex::new(0);

// This type is a thread-safe [`LazyCell`], and can be used in statics.
static DATA: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

async fn task1() {
    println!("Task1 try lock");
    // let _guard = DATA.lock().unwrap();
    let _guard = DATA.lock().await;
    println!("Task1 locked, sleep 5s");
    sleep(Duration::from_secs(5)).await;
    println!("Task1 done");
}

async fn task2() {
    sleep(Duration::from_millis(100)).await;
    println!("Task2 try lock");
    // let _guard = DATA.lock().unwrap();
    let _guard = DATA.lock().await;
    println!("Task2 locked");
}

#[tokio::main]
// #[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    tokio::join!(task1(), task2());
}