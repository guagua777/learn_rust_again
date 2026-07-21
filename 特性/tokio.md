# Tokio 完整入门教程（Tokio 1.x 稳定版）
## 一、Tokio 是什么
Tokio 是 Rust 生态**标准异步运行时**，提供：
1. 多线程调度器（轻量异步任务，类似 Go goroutine）
2. 异步网络、文件、定时器、同步原语
3. `async/await` 执行环境，支撑 Axum/Tonic/Hyper 等网络框架
底层基于 `mio`（epoll/kqueue/io_uring）实现高性能非阻塞 I/O。

### 核心概念速记
- **Future**：异步操作状态机，不 poll 就不会执行
- **Task**：最小调度单元，`tokio::spawn` 创建
- **Runtime**：运行时，驱动所有 Future 执行
- **Reactor**：I/O 事件轮询器，处理网络/文件等待



## 二、环境初始化
### 1. 创建项目
```bash
cargo new tokio-demo
cd tokio-demo
```

### 2. Cargo.toml 依赖（推荐精简 feature，不建议 full）
```toml
# 最小网络开发配置
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "net", "time", "sync", "fs"] }
```
- `rt-multi-thread`：多线程运行时（默认）
- `macros`：启用 `#[tokio::main]`、`#[tokio::test]`
- `full`：包含所有功能，仅本地测试用，生产禁用（体积大）

## 三、Hello Tokio 基础示例
### 示例1：最简 async main
`#[tokio::main]` 自动创建 Runtime，让 main 变成异步函数
```rust
#[tokio::main]
async fn main() {
    println!("Hello Tokio!");
}
```
运行：`cargo run`

### 示例2：手动构建 Runtime（进阶）
```rust
use tokio::runtime::Runtime;

fn main() {
    // 创建多线程运行时
    let rt = Runtime::new().unwrap();
    // 阻塞主线程，执行异步代码
    rt.block_on(async {
        println!("手动管理 Runtime");
    });
}
```

### 单线程运行时（CPU 弱设备/纯 I/O）
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("单线程 Runtime");
}
```

## 四、异步任务 Task 并发
### 1. tokio::spawn 生成后台任务
`spawn` 返回 `JoinHandle`，`.await` 等待任务返回值
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 生成异步任务
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "任务执行完成"
    });

    println!("主线程等待子任务...");
    // 等待任务结束，unwrap 捕获 panic
    let res = handle.await.unwrap();
    println!("子任务返回：{}", res);
}
```

### 2. 多任务并行：join! / try_join!
`join!` 等待全部任务完成，收集所有返回值；**并行执行**，总耗时取最长任务
```rust
use tokio::time::{sleep, Duration};

async fn task1() -> u32 {
    sleep(Duration::from_secs(1)).await;
    10
}
async fn task2() -> u32 {
    sleep(Duration::from_secs(2)).await;
    20
}

#[tokio::main]
async fn main() {
    let (r1, r2) = tokio::join!(task1(), task2());
    println!("r1={}, r2={}", r1, r2); // 总耗时 2s
}
```
- `try_join!`：任一任务返回 Err 立即整体报错，适合错误快速失败场景

### 3. JoinSet：动态批量任务
适合不确定任务数量的场景，自动收集完成任务
```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for i in 0..5 {
        set.spawn(async move {
            sleep(Duration::from_millis(i * 100)).await;
            i
        });
    }
    // 逐个取出完成的任务
    while let Some(res) = set.join_next().await {
        println!("任务结果：{}", res.unwrap());
    }
}
```

## 五、定时器与超时控制
### 1. sleep 异步等待（不要用 std::thread::sleep！会阻塞线程）
```rust
// ❌ 错误：阻塞整个工作线程，所有任务停滞
std::thread::sleep(std::time::Duration::from_secs(1));

// ✅ 正确：让出线程，调度其他任务
use tokio::time::{sleep, Duration};
sleep(Duration::from_secs(1)).await;
```

### 2. timeout 限时执行
超过时间直接返回 Err，防止任务卡死
```rust
use tokio::time::{timeout, Duration};

async fn slow_task() {
    sleep(Duration::from_secs(3)).await;
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(2), slow_task()).await {
        Ok(_) => println!("任务正常完成"),
        Err(_) => println!("任务超时！"),
    }
}
```

### 3. interval 周期定时器
```rust
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut tick = interval(Duration::from_secs(1));
    for _ in 0..3 {
        tick.tick().await;
        println!("每秒触发一次");
    }
}
```

## 六、select! 多路等待（核心工具）
同时等待多个 Future，**谁先就绪执行谁**，常用于：超时、信号、关闭、多通道监听
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let f1 = sleep(Duration::from_secs(1));
    let f2 = sleep(Duration::from_secs(2));

    tokio::select! {
        _ = f1 => println!("1秒任务先完成"),
        _ = f2 => println!("2秒任务先完成"),
    }
}
```

### 实用场景：任务 + 超时
```rust
async fn work() { sleep(Duration::from_secs(3)).await; }

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = work() => println!("工作完成"),
        _ = sleep(Duration::from_secs(2)) => println!("超时退出"),
    }
}
```

## 七、异步 I/O：网络 TCP 服务端+客户端
### 1. TCP Echo 服务端（高并发，每个连接单独 spawn）
```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("监听 8000 端口");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("新连接：{}", addr);
        // 每一条连接生成独立任务，并发处理
        tokio::spawn(async move {
            handle_conn(stream).await;
        });
    }
}

async fn handle_conn(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];
    loop {
        // 异步读
        let n = match stream.read(&mut buf).await {
            Ok(0) => return, // 客户端关闭连接
            Ok(v) => v,
            Err(_) => return,
        };
        // 回写数据
        stream.write_all(&buf[0..n]).await.unwrap();
    }
}
```

### 2. TCP 客户端
```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8000").await?;
    stream.write_all(b"hello tokio").await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("收到回声：{}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
```

### 3. 异步文件读写 tokio::fs
替代 std::fs，非阻塞文件操作
```rust
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 写文件
    fs::write("test.txt", "tokio file async").await?;
    // 读文件
    let content = fs::read_to_string("test.txt").await?;
    println!("文件内容：{}", content);
    Ok(())
}
```

## 八、同步原语（多任务共享数据）
### 1. tokio::sync::Mutex 异步锁（**不要用 std::sync::Mutex**）
std Mutex 阻塞线程，tokio Mutex 是异步等待，让出调度线程
```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Arc 多任务共享，Mutex 内部可变
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let cnt = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut num = cnt.lock().await;
            *num += 1;
        }));
    }
    // 等待所有任务
    for h in handles { h.await.unwrap(); }
    println!("计数：{}", counter.lock().await); // 10
}
```

### 2. 通道 Channel（任务间通信）
#### mpsc：多生产者单消费者（最常用）
```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10); // 缓冲10

    // 生产者任务
    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
    });

    // 消费者接收
    while let Some(v) = rx.recv().await {
        println!("收到：{}", v);
    }
}
```
其他通道：
- `oneshot`：一次性收发（任务返回结果）
- `broadcast`：多生产者多消费者广播
- `watch`：单值订阅（配置热更新）

## 九、阻塞 CPU 密集任务：spawn_blocking
异步线程不适合大量计算，会饿死其他 I/O 任务，用专用阻塞线程池
```rust
#[tokio::main]
async fn main() {
    // 阻塞计算放入 spawn_blocking
    let res = tokio::task::spawn_blocking(|| {
        // 同步密集计算
        let mut s = 0;
        for i in 0..1_000_000 { s += i; }
        s
    }).await.unwrap();
    println!("计算结果：{}", res);
}
```

## 十、核心避坑最佳实践
1. **禁止在 async 代码调用同步阻塞 API**
   `std::thread::sleep`、同步文件读写、同步数据库会卡住工作线程
2. **共享数据必须用 Arc + tokio::sync 原语**
   普通引用无法跨任务生命周期
3. **不要在任务中持有长时间锁**
   Mutex 持有期间不要执行 `.await` 耗时操作
4. **网络服务一定要加 timeout**
   防止空闲连接耗尽资源
5. **生产环境禁用 features = ["full"]**
   按需开启 feature 减小编译体积
6. 任务 panic 只会销毁自身，不会崩溃整个 Runtime，通过 JoinHandle 捕获

## 十一、官方文档参考
1. 英文官方教程：https://tokio.rs/tokio/tutorial
2. 中文文档：https://tokio-cn.github.io/
3. API 文档：https://docs.rs/tokio/latest/tokio/

## 十二、进阶学习路线
1. 掌握 `select!`、`join!` 组合并发控制
2. 通道多模型：broadcast/watch/oneshot 业务场景区分
3. 优雅关闭（shutdown 信号 + select! 退出逻辑）
4. Runtime 自定义调优（工作线程数、阻塞池大小）
5. 基于 Tokio 开发 Axum HTTP、Tonic gRPC 服务

需要我把上面所有代码整理成一份可直接运行的完整工程（分模块：基础任务、TCP服务、通道、锁、超时）吗？










# 一、核心结论
`tokio::sync::Mutex` 的**锁守卫（Guard）持有期间，绝对不能调用 `.await`**，更不能执行耗时异步操作。
原因分两层：
1. 锁守卫是普通同步对象，**不会跨 await 自动释放锁**；
2. 锁长时间占用会阻塞其他任务，直接破坏异步调度并发能力。

## 1. 先搞懂：MutexGuard 是什么
```rust
let mut guard = mutex.lock().await; 
// guard 持有锁，离开作用域才 drop、释放锁
```
`guard` 是 RAII 守卫：只有它被销毁时，锁才释放。
只要变量 `guard` 还存在，锁就一直占着。

## 2. 反例：错误写法（锁内 await，致命问题）
```rust
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let t1 = tokio::spawn({
        let d = Arc::clone(&data);
        async move {
            let mut guard = d.lock().await;
            // ❌ 锁持有中执行 await，锁卡死！
            sleep(Duration::from_secs(3)).await;
            *guard += 1;
        }
    });

    let t2 = tokio::spawn({
        let d = Arc::clone(&data);
        async move {
            let mut guard = d.lock().await; // 要等 3 秒才能拿到锁
            *guard += 1;
        }
    });

    let _ = tokio::join!(t1, t2);
    println!("{}", data.lock().await);
}
```
### 问题拆解
1. **任务1拿到锁，sleep 3秒让出线程，但 `guard` 变量还活着 → 锁不释放**
2. 这3秒内，所有其他想要这把锁的任务全部阻塞排队；
3. 本来异步可以并行大量 I/O，现在变成串行排队，并发彻底废掉；
4. 如果锁内是数据库查询、TCP 请求、文件读写这种长耗时异步，系统直接吞吐量暴跌，甚至出现大量任务堆积、超时雪崩。

## 3. 更深一层：和 std::sync::Mutex 对比，误区澄清
有人会说：同步 Mutex 里也不能 sleep，有什么区别？
- `std::sync::Mutex`：锁内同步阻塞，会**卡死整个工作线程**，整个线程上所有任务停滞；
- `tokio::sync::Mutex`：锁内 await 不会卡死线程，但**锁长期霸占**，所有竞争该锁的任务串行排队，丧失并发。

两种都是严重bug，但危害场景不同。

# 二、为什么锁内 await 锁不会自动释放？
Rust 的 `await` 会把当前任务挂起、让出运行线程，但**局部变量生命周期不变**。
```rust
let guard = mutex.lock().await; // 获取锁
some_async_op().await;          // 挂起任务，但 guard 还在当前栈，不会 drop
// 直到 guard 离开 {} 作用域才释放锁
```
编译器不会智能地在 await 处 drop 守卫，RAII 只认作用域边界，不认 await 断点。

# 三、正确写法：缩小锁临界区，锁只保护纯内存操作
原则：**只在持有锁时做最快的内存读写，所有 IO/睡眠/网络全部放到锁外面**。

## 修正上面的例子
```rust
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let t1 = tokio::spawn({
        let d = Arc::clone(&data);
        async move {
            // 1. 拿到锁，快速修改内存
            let mut guard = d.lock().await;
            *guard += 1;
            drop(guard); // 显式释放锁（或关闭大括号自动释放）
            
            // 2. 锁已经释放，再执行耗时异步
            sleep(Duration::from_secs(3)).await;
        }
    });

    let t2 = tokio::spawn({
        let d = Arc::clone(&data);
        async move {
            let mut guard = d.lock().await;
            *guard += 1;
            // 大括号结束，guard 自动销毁，锁释放
        }
    });

    let _ = tokio::join!(t1, t2);
    println!("{}", data.lock().await); // 2，且无等待阻塞
}
```
### 两种释放锁方式
1. 手动 `drop(guard);` 立刻释放；
2. 用内部代码块缩小作用域：
```rust
{
    let mut guard = data.lock().await;
    *guard += 1;
} // 此处 guard 销毁，锁释放
// 后面随便 await
sleep(...).await;
```

# 四、典型危险场景（日常开发高频踩坑）
以下操作绝对不能放在 `lock().await` 守卫内部：
1. `tokio::time::sleep / timeout / interval`
2. 网络操作：`TcpStream` 读写、HTTP 请求、gRPC 调用
3. 文件操作 `tokio::fs` 读写大文件
4. 数据库异步查询（sqlx、diesel-async）
5. 通道收发 `tx.send().await`、`rx.recv().await`
6. 其他同步原语操作：另一把锁、broadcast 订阅等

# 五、特殊场景：必须先读锁内数据再异步操作，怎么处理？
场景：锁里存用户信息，需要拿到用户名后发起HTTP请求。
错误：
```rust
let guard = user.lock().await;
let name = &guard.name;
http_get(format!("https://api/{}", name)).await; // 锁还占着
```
正确：**把需要的数据拷贝出来，立刻释放锁**
```rust
let guard = user.lock().await;
let name = guard.name.clone(); // 拷贝数据到栈
drop(guard); // 释放锁
http_get(format!("https://api/{}", name)).await;
```

# 六、延伸：读写锁 RwLock 同一规则
`tokio::sync::RwLock` 读守卫、写守卫同样遵循：
守卫持有期间不能 await，必须缩小临界区。
读锁长时间持有也会阻塞写任务，一样会拖垮并发。

# 七、一句话总结记忆
Mutex 守卫只有出作用域才释放；
锁内只做内存读写，所有 `.await` 耗时操作全部挪到锁外面，避免锁长期占用导致任务串行排队、并发失效。