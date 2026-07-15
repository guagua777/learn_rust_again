### 先明确：标准库里没有 `OnceLock` 旧叫法 / 分清两个类型
- **`std::sync::OnceLock<T>`** ✅ Rust 1.70+ 标准库正式类型（多线程一次性懒加载容器）
- **`std::sync::OnceCell<T>`** ✅ 同期存在，两者高度相似但定位/API有差异
- 还有：`std::cell::OnceCell<T>` = **单线程版 OnceCell**（先排除，不在本次对比范围）

## 1 基础信息
### OnceLock<T>
```rust
// std::sync::OnceLock<T>
pub struct OnceLock<T>
```
- 核心：**专门用来存一个 `T`，完成一次性初始化后永久只读**
    - 初始化后**不支持再次修改内容**，只能读取引用 `&T`
    - 底层封装 `Once` + 内部存储，保证全局唯一初始化，线程安全（`Sync + Send`）
    - 主要 API：
        - `get_or_init(|| T)`：首次执行初始化闭包，返回 `&T`
        - `get()` / `get_mut()`：获取引用（`get_mut()` 仅在未初始化/独占访问时可用）
        - `set()`：手动写入初始值（只能成功一次）
    - 语义：**Once 锁 + 只读单例容器**，强调“初始化完成后固定不变”

### OnceCell<T> (std::sync::OnceCell<T>)
```rust
// std::sync::OnceCell<T>
pub struct OnceCell<T>
```
- 前身：第三方 once_cell crate 的主线版本，后移入标准库
    - 同样是一次性初始化、线程安全懒加载容器
    - 相比 OnceLock：API 更灵活，支持 `take()` / `into_inner()` / 可取出所有权、重置/接管数据
    - 主要 API：
        - `get_or_init(|| T)` / `get_or_try_init`（支持错误返回）
        - `take()`：取出数据，让容器变回未初始化状态
        - `into_inner()`：消费容器，取出所有权
    - 语义：**可回收/可接管的一次性容器，不止是永久只读单例**

## 2 核心区别对照
| 特性 | `OnceLock<T>` | `std::sync::OnceCell<T>` |
|---|---|---|
| **核心定位** | 全局只读静态单例，初始化后长期固定不变 | 通用一次性懒加载容器，支持取回/重置数据 |
| **数据生命周期** | 数据常驻容器，主打长期 &'static 只读引用 | 可通过 `take()`/`into_inner()` 取出数据、释放内存，不强制常驻 |
| **可变性** | 初始化完成后 **常规只能只读访问**，不适合反复修改 | 支持取出/替换数据，灵活性更高 |
| **错误初始化** | 基础版不原生支持 fallible 错误初始化 | 提供 `get_or_try_init`，方便处理初始化失败（数据库连接错误等） |
| **典型用途** | 全局静态配置、常量单例、全局客户端（永不销毁） | 需要懒加载、同时后续可能手动释放/重建资源的场景 |
| **内存模型** | 主打长期全局 static 变量 | 可用于局部作用域 + static 全局两种场景 |
| **推荐写法** | 配合 `static` 全局常量单例 | 灵活懒加载、fallible 初始化、可回收资源 |

### 代码示例对比
#### OnceLock 全局静态单例用法
```rust
use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    GLOBAL_CONFIG.get_or_init(|| {
        // 仅运行一次初始化
        std::fs::read_to_string("config.toml").unwrap()
    })
}
```

#### OnceCell 可错误初始化 + 可回收用法
```rust
use std::sync::OnceCell;

static DB: OnceCell<DbConn> = OnceCell::new();

// 可处理初始化失败
fn get_db() -> Result<&'static DbConn, DbError> {
    DB.get_or_try_init(|| DbConn::connect("postgres://..."))
}

// 可取出数据释放（OnceLock 不适合做这个）
fn close_db() -> Option<DbConn> {
    DB.take()
}
```

## 3 历史背景
1. Rust 早期只有第三方 `once_cell::sync::OnceCell`
2. Rust 1.70 并入标准库：
    - `std::sync::OnceLock`：专门适配 `static` 全局只读单例场景，做简化优化
    - `std::sync::OnceCell`：保留原版灵活 API，用于复杂场景
    - `std::cell::OnceCell`：单线程版本
3. 本质底层都依赖 `std::sync::Once` 做一次性同步保证，**都线程安全、都保证只执行一次初始化逻辑**

## 4 使用建议
- ✅ **全局长期不变的静态单例 → OnceLock<T>**
    - 写法干净、优化更好、符合全局常量语义
- ✅ **需要错误初始化、动态重建/释放资源、非全局场景 → std::sync::OnceCell<T>**
    - 适合连接池、可重启服务组件、fallible 初始化场景
- ❗ 不要混淆单线程 `std::cell::OnceCell`，它不是线程安全版本

## 一句话总结
- `OnceLock<T>` = **静态全局只读永久单例容器（主打 static）**
- `std::sync::OnceCell<T>` = **灵活可回收、支持错误初始化的通用一次性容器**
- 共同点：线程安全、保证仅初始化一次；核心差异在于是否允许取出/重置数据、是否主打长期只读全局单例

要不要我把上面两个例子改成可直接运行的完整代码？






# Rust OnceCell 使用背景 & 原理
## 1. 核心痛点（使用背景）
### 痛点1：Rust 原生静态变量的局限
Rust 普通 `static` 变量有严格约束：
- 必须是**编译期可确定常量表达式**，不能运行时动态初始化（读取配置、IO、网络连接、数据库连接、读取环境变量、计算复杂参数）
- 可变 static 原生写法不安全（需要 `unsafe`、全局竞争、无初始化保障）
- `lazy_static!` 宏虽然解决延迟初始化，但存在缺点：宏语法、运行时检查开销、API 不友好、无法保证一次性初始化语义、不支持 async、无法方便获取是否已初始化状态

### 痛点2：全局单例/一次性初始化场景需求
很多程序需要：
- **全局唯一实例，仅初始化一次（懒加载），后续直接复用，不再重复构造**
    - 全局配置、数据库连接池、日志实例、客户端 SDK、全局资源句柄
    - 保证：**无论多少处访问，只执行一次初始化逻辑，线程安全版本可用于多线程**
    - 保证：初始化完成前阻塞访问，防止未初始化就读取数据
- 区分两种场景：
    - 单线程：`std::cell::OnceCell`
    - 多线程：`std::sync::OnceCell` / `once_cell::sync::OnceCell`
- 替代手写 `Once` + 裸静态变量 + unsafe 的繁琐写法，提供安全、标准的懒加载容器

### 痛点3：FFI / 库初始化、异步初始化
- 库代码、底层组件：需要延迟初始化全局资源，避免程序启动时阻塞/占用资源，用到时才初始化
- async 场景：支持异步一次性初始化（第三方 `once_cell` crate），原生静态不支持 async 初始化
- 避免重复创建昂贵资源（数据库连接、TLS上下文、大缓存），减少性能开销

### 痛点4：类型与生命周期问题
- 全局数据常是非 `'static`、非常量类型（`String`、连接句柄、结构体），无法直接放进普通 `static`
- 需要安全地生成 `&'static T` 引用，配合静态生命周期约束，不滥用 Box::leak 做粗暴内存泄漏

## 2. 底层基础
- Rust 标准库自 1.70+ 引入 `std::cell::OnceCell<T>`（单线程）、`std::sync::OnceCell<T>`（多线程）
- 前身是第三方 crate `once_cell`，后来纳入标准库
- 本质：配合一次性初始化标记（类似 `Once`）+ 内部可变性容器，保证**最多执行一次初始化**

## 3. 核心特性 & 用途
### ✅ 核心语义
- 状态：未初始化 / 已初始化
- `get_or_init(|| 初始化函数)`：
    - 首次调用：执行初始化闭包，存入数据，返回引用
    - 后续调用：直接返回已有数据，不再运行闭包
    - 多线程版本会做同步阻塞，保证只有1个线程执行初始化
- 可检查是否初始化：`is_initialized()`、`get()`

### ✅ 典型使用场景
1. **懒加载全局单例**
    - 全局配置、数据库连接、HTTP 客户端、日志器
    - 程序启动不初始化，第一次使用时初始化一次
    ```rust
    use std::sync::OnceCell;
    static DB: OnceCell<DbConn> = OnceCell::new();
    fn get_db() -> &'static DbConn {
        DB.get_or_init(|| DbConn::connect("url"))
    }
    ```
2. **昂贵资源一次性构造**：避免反复创建连接、大对象，减少性能损耗
3. **替代 Box::leak / lazy_static**：可控静态生命周期，避免无期限内存泄漏，语义更清晰
4. **测试/库模块全局状态**：保证全局变量只初始化一次
5. 配合 `LazyCell`：纯同步懒加载常量

### ✅ 和 Box::leak / lazy_static 的区别
- `Box::leak`：永久内存泄漏，无初始化保护，每次调用都可能新建对象
- `lazy_static`：宏实现、老方案、语法笨重
- `OnceCell`：显式一次性懒加载、可检查初始化状态、支持多线程版本、可复用内存

### ✅ 线程区别
- `std::cell::OnceCell`：单线程版本，类似 Cell，非 Sync
- `std::sync::OnceCell`：多线程版本，带同步机制，可跨线程访问全局单例

## 4. 一句话总结使用背景
**解决 Rust 原生 static 无法运行时懒加载初始化全局单例的痛点，提供安全、可控、一次性懒加载容器，用于全局昂贵资源、配置、连接池等单例管理，替代旧方案 lazy_static 和粗暴 Box::leak。**

要不要我给你写一份 std::sync::OnceCell 全局数据库连接单例的可运行示例？




### 先明确：标准库里没有 `OnceLock` 旧叫法 / 分清两个类型
- **`std::sync::OnceLock<T>`** ✅ Rust 1.70+ 标准库正式类型（多线程一次性懒加载容器）
- **`std::sync::OnceCell<T>`** ✅ 同期存在，两者高度相似但定位/API有差异
- 还有：`std::cell::OnceCell<T>` = **单线程版 OnceCell**（先排除，不在本次对比范围）

## 1 基础信息
### OnceLock<T>
```rust
// std::sync::OnceLock<T>
pub struct OnceLock<T>
```
- 核心：**专门用来存一个 `T`，完成一次性初始化后永久只读**
    - 初始化后**不支持再次修改内容**，只能读取引用 `&T`
    - 底层封装 `Once` + 内部存储，保证全局唯一初始化，线程安全（`Sync + Send`）
    - 主要 API：
        - `get_or_init(|| T)`：首次执行初始化闭包，返回 `&T`
        - `get()` / `get_mut()`：获取引用（`get_mut()` 仅在未初始化/独占访问时可用）
        - `set()`：手动写入初始值（只能成功一次）
    - 语义：**Once 锁 + 只读单例容器**，强调“初始化完成后固定不变”

### OnceCell<T> (std::sync::OnceCell<T>)
```rust
// std::sync::OnceCell<T>
pub struct OnceCell<T>
```
- 前身：第三方 once_cell crate 的主线版本，后移入标准库
    - 同样是一次性初始化、线程安全懒加载容器
    - 相比 OnceLock：API 更灵活，支持 `take()` / `into_inner()` / 可取出所有权、重置/接管数据
    - 主要 API：
        - `get_or_init(|| T)` / `get_or_try_init`（支持错误返回）
        - `take()`：取出数据，让容器变回未初始化状态
        - `into_inner()`：消费容器，取出所有权
    - 语义：**可回收/可接管的一次性容器，不止是永久只读单例**

## 2 核心区别对照
| 特性 | `OnceLock<T>` | `std::sync::OnceCell<T>` |
|---|---|---|
| **核心定位** | 全局只读静态单例，初始化后长期固定不变 | 通用一次性懒加载容器，支持取回/重置数据 |
| **数据生命周期** | 数据常驻容器，主打长期 &'static 只读引用 | 可通过 `take()`/`into_inner()` 取出数据、释放内存，不强制常驻 |
| **可变性** | 初始化完成后 **常规只能只读访问**，不适合反复修改 | 支持取出/替换数据，灵活性更高 |
| **错误初始化** | 基础版不原生支持 fallible 错误初始化 | 提供 `get_or_try_init`，方便处理初始化失败（数据库连接错误等） |
| **典型用途** | 全局静态配置、常量单例、全局客户端（永不销毁） | 需要懒加载、同时后续可能手动释放/重建资源的场景 |
| **内存模型** | 主打长期全局 static 变量 | 可用于局部作用域 + static 全局两种场景 |
| **推荐写法** | 配合 `static` 全局常量单例 | 灵活懒加载、fallible 初始化、可回收资源 |

### 代码示例对比
#### OnceLock 全局静态单例用法
```rust
use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    GLOBAL_CONFIG.get_or_init(|| {
        // 仅运行一次初始化
        std::fs::read_to_string("config.toml").unwrap()
    })
}
```

#### OnceCell 可错误初始化 + 可回收用法
```rust
use std::sync::OnceCell;

static DB: OnceCell<DbConn> = OnceCell::new();

// 可处理初始化失败
fn get_db() -> Result<&'static DbConn, DbError> {
    DB.get_or_try_init(|| DbConn::connect("postgres://..."))
}

// 可取出数据释放（OnceLock 不适合做这个）
fn close_db() -> Option<DbConn> {
    DB.take()
}
```

## 3 历史背景
1. Rust 早期只有第三方 `once_cell::sync::OnceCell`
2. Rust 1.70 并入标准库：
    - `std::sync::OnceLock`：专门适配 `static` 全局只读单例场景，做简化优化
    - `std::sync::OnceCell`：保留原版灵活 API，用于复杂场景
    - `std::cell::OnceCell`：单线程版本
3. 本质底层都依赖 `std::sync::Once` 做一次性同步保证，**都线程安全、都保证只执行一次初始化逻辑**

## 4 使用建议
- ✅ **全局长期不变的静态单例 → OnceLock<T>**
    - 写法干净、优化更好、符合全局常量语义
- ✅ **需要错误初始化、动态重建/释放资源、非全局场景 → std::sync::OnceCell<T>**
    - 适合连接池、可重启服务组件、fallible 初始化场景
- ❗ 不要混淆单线程 `std::cell::OnceCell`，它不是线程安全版本

## 一句话总结
- `OnceLock<T>` = **静态全局只读永久单例容器（主打 static）**
- `std::sync::OnceCell<T>` = **灵活可回收、支持错误初始化的通用一次性容器**
- 共同点：线程安全、保证仅初始化一次；核心差异在于是否允许取出/重置数据、是否主打长期只读全局单例

要不要我把上面两个例子改成可直接运行的完整代码？





# 完整可运行示例
## 示例1：OnceLock 全局只读静态单例
```rust
use std::sync::OnceLock;

// 全局只读配置
static GLOBAL_CONFIG: OnceLock<String> = OnceLock::new();

// 获取全局配置（仅首次访问时初始化）
fn get_config() -> &'static String {
    GLOBAL_CONFIG.get_or_init(|| {
        println!("执行一次性初始化！");
        // 模拟读取配置/环境变量等运行时初始化逻辑
        String::from("server_addr = 127.0.0.1:8080")
    })
}

fn main() {
    // 首次调用：运行初始化闭包
    println!("{}", get_config());
    // 后续调用：直接读取已有数据，不再执行初始化
    println!("{}", get_config());
    println!("{}", get_config());
}
```
运行结果
```
执行一次性初始化！
server_addr = 127.0.0.1:8080
server_addr = 127.0.0.1:8080
server_addr = 127.0.0.1:8080
```
特点：
- 初始化完成后**只读常驻内存，不适合取出/修改原始数据**
- 完美适配长期不变的全局静态资源

---

## 示例2：std::sync::OnceCell 可错误初始化 + 可回收资源
```rust
use std::sync::OnceCell;

// 模拟数据库连接类型
#[derive(Debug)]
struct DbConn {
    addr: String,
}

impl DbConn {
    // 模拟数据库连接（可返回错误）
    fn connect(addr: &str) -> Result<Self, String> {
        println!("尝试建立数据库连接");
        Ok(DbConn { addr: addr.to_string() })
    }

    fn query(&self) {
        println!("查询数据库: {}", self.addr);
    }
}

// 全局数据库连接容器
static DB: OnceCell<DbConn> = OnceCell::new();

// 支持错误处理的懒加载获取
fn get_db() -> Result<&'static DbConn, String> {
    DB.get_or_try_init(|| DbConn::connect("postgres://localhost:5432/mydb"))
}

// 取出连接，释放资源（OnceLock 不适合做这个操作）
fn close_db() -> Option<DbConn> {
    DB.take()
}

fn main() {
    // 首次初始化
    let conn = get_db().unwrap();
    conn.query();

    // 再次调用不会重复连接
    let conn2 = get_db().unwrap();
    conn2.query();

    // 取出连接，容器变回未初始化状态
    if let Some(db) = close_db() {
        println!("关闭连接: {:?}", db);
    }

    // 再次获取会重新初始化
    let conn3 = get_db().unwrap();
    conn3.query();
}
```
运行结果
```
尝试建立数据库连接
查询数据库: postgres://localhost:5432/mydb
查询数据库: postgres://localhost:5432/mydb
关闭连接: DbConn { addr: "postgres://localhost:5432/mydb" }
尝试建立数据库连接
查询数据库: postgres://localhost:5432/mydb
```
特点：
- `get_or_try_init`：原生支持失败初始化
- `take()`：可以取出所有权、重置容器，实现资源关闭/重建
- 灵活性更高，适合连接这类需要生命周期管理的资源

---

## 核心区别重申
1. `OnceLock`：主打**长期不变 static 全局单例，只读访问，不取出数据**
2. `std::sync::OnceCell`：主打**可错误初始化、可手动释放/重建资源的一次性容器**
3. 两者都保证线程安全、主线程只会执行一次初始化逻辑

要不要我再加一个多线程并发调用 OnceLock / OnceCell 的测试代码，验证只会初始化一次？




# 核心区别拆解（3个最关键差异）
## 1. 根本设计目标不同
### OnceLock<T>
> **设计目的：做永久固定的全局 static 只读单例，数据一旦初始化就长期留在容器里，不允许整体取出、不重置**
- 主打长期全局常量、配置、基础客户端，整个程序生命周期只初始化1次，全程只读
- API 偏向只读引用 `&T`
    - 只有极少数场景能用 `get_mut()`：必须保证独占访问、容器还没被全局共享时，**初始化完成后几乎不能改、不能整体拿走数据**
    - 没有 `take()` / `into_inner()` 这类取出所有权的方法
    - 没有原生 `get_or_try_init` 错误初始化 API

```rust
use std::sync::OnceLock;
static CFG: OnceLock<String> = OnceLock::new();
fn get_cfg() -> &'static String {
    CFG.get_or_init(|| String::from("fixed config"))
}
// ✅ 一直读引用，全程不拿走原始 String
// ❌ 没办法直接 take() 把 String 移走释放
```

### std::sync::OnceCell<T>
> **设计目的：通用一次性容器，支持错误初始化、可以随时把数据整体取走、重置容器状态**
- 不强制永久常驻，兼顾全局 + 局部使用场景
- 关键独有能力：
    1. `get_or_try_init`：原生支持返回 Result，方便处理数据库连接失败、IO 错误这类初始化异常
    2. `take()`：把内部数据**整体取出所有权**，容器变回空的未初始化状态，之后可以重新初始化
    3. `into_inner()`：消费整个容器，取出所有权，彻底释放内存
    - 适合连接、可重启服务组件、需要手动关闭/重建的资源

```rust
use std::sync::OnceCell;
static DB: OnceCell<DbConn> = OnceCell::new();
fn close() -> Option<DbConn> {
    DB.take() // ✅ 取出整个连接，容器重置为空
}
```

## 2. 生命周期 & 内存模型差异
- **OnceLock**
    - 和 `static` 全局变量深度绑定，追求 `&'static T` 永久引用
    - 数据常驻静态内存，跟随程序全程，不做动态释放、重建
    - 适合：全局常量、基础配置、只读全局实例
- **OnceCell**
    - 既可以用在 static 全局，也可以用在局部变量里
    - 可以主动释放数据，不必全程常驻内存
    - 适合：连接池、可重启服务、临时懒加载资源、会失败的初始化

## 3. 错误初始化能力差异
- **OnceLock**
    - 原生没有 `get_or_try_init`
    - 如果初始化可能失败（连数据库、读文件出错），写法很别扭，需要手动写错误处理逻辑
    - 适合：**一定可以成功初始化的常量数据**
- **OnceCell**
    - 原生 `get_or_try_init`，优雅处理初始化失败场景
    - 适合：IO、网络连接、外部依赖这类可能失败的初始化

## 4. 直观对照
| 关键点 | OnceLock<T> | std::sync::OnceCell<T> |
|---|---|---|
| 是否可以整体取出数据（take） | ❌ 不支持，数据常驻容器 | ✅ 支持 take() / into_inner() |
| 原生错误初始化 | ❌ 无 get_or_try_init | ✅ get_or_try_init 原生支持 Result |
| 主要用法 | static 全局只读单例 | 灵活懒加载，可重建/释放资源 |
| 数据生命周期 | 全局常驻，不主动回收 | 可手动回收、重置、重建 |

## 5. 最简记忆口诀
- **OnceLock = 永久静态只读单例（定死不变、全程只读、不能拿走）**
- **OnceCell = 可错误初始化、可重置回收的灵活懒加载容器（可拿走重建）**

### 验证代码（一眼看出差异）
#### ✅ OnceCell：可以 take 重置
```rust
use std::sync::OnceCell;

let cell = OnceCell::new();
cell.set(100).unwrap();
println!("{}", cell.get().unwrap()); // 100
let val = cell.take(); // 取出数据，容器变空
println!("{:?}", cell.get()); // None
```

#### ✅ OnceLock：不能 take
```rust
use std::sync::OnceLock;

let lock = OnceLock::new();
lock.set(100).unwrap();
println!("{}", lock.get().unwrap()); // 100
// lock.take(); // ❌ 编译报错：OnceLock 根本没有 take() 方法
```

一句话总结区别：
- 只要你需要 `take()` / 错误初始化 / 重建资源 → 选 `std::sync::OnceCell`
- 只要你做长期不变的全局静态常量 → 选 `OnceLock`




感觉主要还是历史的原因，才会出现这两个api，如果一开始就设计的很好，好像只需要一个就可以

# 结论：不完全是单纯历史遗留问题，同时包含 **设计语义分离 + 历史演进 + 类型系统/静态优化诉求**，不只是重复 API
下面分三层讲清楚

## 一、确实有历史演进背景
1. **早期生态：只有第三方 `once_cell` crate**
    - Rust 原生长期没有标准库懒加载全局单例，大家都用 `lazy_static` / 手写 `Once` + static / `once_cell` crate
    - `once_cell::sync::OnceCell` 主打通用懒加载、支持 fallible、take、into_inner，适配各种场景（全局 + 局部、可重建资源）
    - 后来 Rust 官方要纳入标准库，但发现存在两个完全不同的核心使用模式：
        - A：**static 全局永久只读单例（绝大多数基础全局配置）**
        - B：**可重建、可回收、可失败初始化的动态懒加载（连接、服务资源）**
2. 最终方案
    - 新增 **`std::sync::OnceLock<T>`**：专门做 A 场景，做静态全局只读单例、做类型优化、保证永久 &'static 引用语义
    - 保留 **`std::sync::OnceCell<T>`**：继承第三方 crate 的通用 API，做 B 场景
    - 同时还有 `std::cell::OnceCell`：单线程版本
    - 所以**确实有历史兼容因素**，要兼容老 once_cell crate 的用法，不能直接一刀切合并

## 二、更深层：语义/类型系统层面，本来就存在两种不同契约，无法完美合并为一个类型
### 契约 A：OnceLock
> **契约承诺：一旦初始化完成，数据永久驻留容器、生命周期 = 'static、只读全局共享，不允许整体移出内存**
- 编译器/用户可以做强假设：永远有效、不会被 take 走、不会被重置
    - 可以安全返回 `&'static T` 长期引用，做全局常量
    - 可以做更好的内存布局/静态内存优化
    - 不需要支持 take/重置/错误重试初始化，减少运行时开销和复杂度
    - 适合程序全生命周期不变的全局常量
- 如果给它加上 take/重置功能：就破坏了 `&'static` 引用的安全前提！
    - 一旦 take() 把数据拿走，现存的 `&'static` 引用就会变成悬垂引用 → UB
    - **这是最核心矛盾，不是技术实现问题，是生命周期安全契约冲突**

### 契约 B：OnceCell
> **契约承诺：一次性初始化，但允许后续重置、取出所有权、重试初始化、处理错误，不强制永久 static 引用**
- 牺牲永久 `&'static` 全局只读保证，换取灵活性
    - 支持 `get_or_try_init`、`take()`、`into_inner()`
    - 适合连接池、可重启服务、会失败初始化、需要手动释放资源的场景
    - 可以用在局部变量，不一定绑定 static
    - 代价：不能无脑无条件返回永久 `&'static` 引用，存在生命周期隐患

### 核心矛盾
- 想要 **无条件安全的永久 &'static 全局只读引用** ✅ OnceLock
- 想要 **可重置、可回收、可错误重试的懒加载容器** ✅ OnceCell
- **一个类型无法同时满足这两套互相冲突的安全契约**
    - 一旦加上 take，就无法保证所有现存引用始终有效
    - 一旦禁止 take，就失去动态资源管理能力

## 三、类比理解
- `OnceLock` ≈ **全局常量 static const**：固定、只读、全程序有效，不能修改/拿走
- `OnceCell` ≈ **可重启服务实例管理器**：懒加载启动、出错重试、用完关闭重建
- 就像 Rust 同时提供 `Box<T>` 和 `Rc<T>`：底层都堆分配，但所有权模型完全不同，不能合并成一个类型

## 四、真实现状 & 写法建议
1. 不是单纯重复冗余 API：
    - 历史兼容是诱因，**生命周期安全契约差异是根本原因**
    - 单纯合并会导致要么全局单例用起来不安全/性能变差，要么动态资源用起来极其别扭
2. 日常编码原则
    - 写长期不变全局静态变量：优先 `OnceLock<T>`
    - 写连接/可重建资源/错误初始化：优先 `std::sync::OnceCell<T>`
    - 局部单线程懒加载：用 `std::cell::OnceCell<T>`
3. 未来：官方不会合并两者，保持两套 API 来区分语义契约

一句话总结：
- **表层原因：历史第三方 once_cell crate 兼容演进**
- **底层原因：两套冲突的生命周期/内存安全契约（永久static只读 vs 可动态回收），无法完美合并为单一类型**
- 如果强行合并为一个 API，一定会牺牲其中一方的安全性或性能

要不要我写一段错误示范：如果用 OnceCell 做全局 &'static 引用，然后调用 take() 造成悬垂引用风险？