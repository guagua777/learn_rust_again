### ✅ `thread::park()` — Rust 线程阻塞函数
所属模块：`std::thread`
```rust
use std::thread;

thread::park();
```

#### 1. 核心作用
**阻塞当前线程，让它进入休眠等待状态，直到被唤醒**
- 线程会卡住不再继续往下执行，**主动让出 CPU**，不忙等
- 本质：基于线程的**等待标记（unpark 标记）**机制
    - 如果此前已经调用过 `thread::unpark(&线程句柄)`：`park()` **不会阻塞，直接立刻返回**（消耗掉这个提前的唤醒信号）
    - 没有预先 unpark：线程挂起，等待被 `unpark` 唤醒 / 被操作系统信号唤醒

#### 2. 配套 API：`thread::Thread::unpark()`
- 用法：保存目标线程句柄，之后调用 `unpark()` 唤醒它
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        println!("子线程：准备 park 等待唤醒");
        thread::park(); // 阻塞子线程
        println!("子线程：已被唤醒，继续运行");
    });

    // 主线程延迟后唤醒子线程
    thread::sleep(Duration::from_millis(1000));
    handle.thread().unpark(); 

    handle.join().unwrap();
}
```

#### 3. 关键特性
- ✅ **不是定时阻塞**（和 `thread::sleep` 不一样）
  - `sleep`：固定时长休眠，到期自动恢复，不受 unpark 控制
  - `park`：无限阻塞，直到显式 `unpark` / 外部信号唤醒
- ✅ **有一次性令牌机制**：unpark 相当于存了一个唤醒令牌，后续一次 park 会直接消费令牌、不阻塞
- ✅ 可被非 Rust 层面信号唤醒（不是只有 unpark 能唤醒），不能保证只由 unpark 唤醒
- ✅ 常用于：简易线程等待、实现自定义同步/自旋锁、生产者消费者模型

#### 4. 衍生版本
- `std::thread::park_timeout(dur)`：阻塞最多指定时长，超时自动恢复，兼顾等待 + 防死锁
```rust
// 最多等待 2 秒，超时自动继续
thread::park_timeout(Duration::from_secs(2));
```

#### 一句话总结
`thread::park()` = **让当前线程暂停运行、等待被 unpark 唤醒的阻塞方法，用于手动控制线程执行节奏**

要不要我顺便对比 park / park_timeout / sleep / join 的区别？




# Rust 4种线程等待方式对比
## 1. `thread::park()`
```rust
std::thread::park()
```
- 阻塞：**无限阻塞当前线程**
- 唤醒方式：
  - `unpark()` 调用（消耗唤醒令牌）
  - OS 信号/外部事件（可能意外唤醒！）
  - ✅ 存在提前 `unpark` 令牌时：直接返回，不阻塞
- CPU：休眠不忙等，几乎无CPU占用
- 用途：精细手动线程调度、自定义同步逻辑
- 缺点：可能被非预期信号唤醒；会死锁如果永远没人 unpark
- 返回：无返回值

## 2. `thread::park_timeout(dur: Duration)`
```rust
std::thread::park_timeout(Duration::from_secs(2))
```
- 阻塞：阻塞**最多指定时长**，超时自动恢复
- 唤醒方式：
  - `unpark()` 提前唤醒
  - 超时自动唤醒
  - OS信号意外唤醒
- CPU：休眠不忙等
- 返回值：`ParkResult`，可区分是超时唤醒还是被 unpark 唤醒
- 用途：等待 + 防死锁兜底，限时等待线程信号

## 3. `thread::sleep(dur: Duration)`
```rust
std::thread::sleep(Duration::from_secs(2))
```
- 阻塞：**固定时长休眠，到期必然恢复，不受 unpark 控制**
- 唤醒方式：只能等时间走完，`unpark()` 无效
- CPU：休眠不忙等
- 用途：单纯延时、固定节奏轮询
- 缺点：无法提前被程序主动唤醒；精度受操作系统调度影响，不是精准计时
- 返回：无返回值

## 4. `handle.join()`
```rust
handle.join().unwrap()
```
- 阻塞：**阻塞当前线程，等待目标子线程彻底执行完毕、正常结束/panic**
- 唤醒方式：仅当目标线程完全退出时才返回
- CPU：休眠等待
- 用途：等待子线程整体任务完成、收集返回结果/处理异常
- 特点：
  - 绑定整个线程生命周期，不是暂停线程、不是中途唤醒
  - 可拿到子线程返回值，也会传播子线程 panic
  - 只能等整个线程结束，不能中途暂停恢复运行

# 核心区别速览表
| API | 核心机制 | 是否可提前unpark唤醒 | 是否自动超时 | 用途 |
|---|---|---|---|---|
| `park()` | 线程唤醒令牌模型 | ✅ | ❌（永久阻塞） | 暂停线程、手动同步 |
| `park_timeout()` | 线程唤醒令牌模型 | ✅ | ✅ | 限时等待 + 防死锁 |
| `sleep()` | 固定时长休眠 | ❌ | ✅ | 单纯延时 |
| `join()` | 等待线程结束 | ❌ | ❌ | 等待整个子线程任务完成 |

# 最佳实践建议
- ✅ 手动暂停线程等待信号 → `park()` / `park_timeout`
- ✅ 单纯做延迟、定时轮询 → `sleep`
- ✅ 等待子线程全部任务跑完、收结果 → `join`
- ✅ 生产代码尽量不用裸 `park()`，优先加超时 `park_timeout` 避免死锁
- ✅ 注意：`park` 存在**虚假唤醒风险**（非unpark原因唤醒），业务代码最好配合循环条件检查

要不要我写一份可直接运行的示例代码，把这4种写法都放进去？




### 核心结论
**持有 Mutex 锁的时候调用 `thread::park()` 是非常危险、不推荐的写法，分情况细说：**

---

## 1. 基础行为：锁会 **保持持有，不会自动释放**
Rust 的 `std::sync::Mutex` 是**绑定守卫（MutexGuard）生命周期的 RAII 锁**
```rust
use std::sync::Mutex;
use std::thread;

let m = Mutex::new(0);
let guard = m.lock().unwrap(); // ✅ 加锁，持有 MutexGuard

thread::park(); // ❗ 线程卡住休眠，guard 仍然有效 → 锁持续被占用

*guard = 123;
```
- `park()` 只是**阻塞当前线程休眠，不会 drop 掉 MutexGuard**
- ✅ 线程休眠期间：**这个 Mutex 一直被当前线程霸占、锁住不放**
- ✅ CPU 不忙等，确实休眠了
- ❌ **其他线程永远拿不到这把 Mutex，直接死锁 / 全线阻塞**
- ❗ 还有虚假唤醒问题：线程可能被意外唤醒，继续拿着锁往下跑

---

## 2. 典型坏场景（死锁）
- 主线程拿着 Mutex → `park()` 等待子线程唤醒自己
- 而子线程**需要获取同一个 Mutex 才能执行 unpark() 唤醒主线程**
    - 主线程：持锁 → park，锁被卡住不放
    - 子线程：请求 lock → 永久阻塞
    - **互相等待 = 死锁，程序卡死**

---

## 3. 和 Condvar（条件变量）做正确对比
条件变量 `Condvar` 的设计就是专门解决“持锁等待”：
```rust
// ✅ Condvar.wait 会：自动释放 Mutex → 阻塞等待 → 被唤醒后重新获取锁
condvar.wait(guard).unwrap();
```
- `Condvar.wait`：**主动释放锁再阻塞，避免锁长期霸占**
- `park`：**不会释放锁！完全没有这个机制** → 不能替代 Condvar

---

## 4. 其他隐患
1. **超时版本 `park_timeout` 也一样：锁依然持有，只是会超时醒来，锁依然被占着直到 guard 被 drop**
2. 虚假唤醒：即使正常 unpark，也要记得检查业务条件；但首要隐患还是**长期占用锁造成死锁**
3. 线程崩溃场景：
    - 如果 park 的线程异常退出（非正常流程），只要 MutexGuard 被 drop，锁才会释放
    - 只要线程还卡在 park，锁就一直占用

---

## 5. 正确做法
- ❌ 不要直接在持有 MutexGuard 的作用域里调用 `park()` / `park_timeout()`
- ✅ 先 `drop(guard);` 释放锁，再 park
  ```rust
  drop(guard); // 主动释放锁
  thread::park();
  ```
- ✅ 如果是同步等待场景：优先用 `Condvar + Mutex` 标准组合，不要裸写 park + Mutex
- ✅ 若一定要用 park，保证等待期间**不持有任何全局/共享 Mutex 守卫**

---

一句话总结：
**park 不会释放 Mutex 锁，锁会一直被卡住的线程持有，极易造成其他线程无法加锁而死锁；千万不要在持有共享 MutexGuard 时直接 park。**

要不要我写一段错误版（死锁）和正确版（drop锁再park）对照代码？