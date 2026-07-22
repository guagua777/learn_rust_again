# 核心先理清一个巨大误区
> ❌ 错误理解：我本地 `cargo build` 编译出来的库文件，发给别人，别人再去开关 feature。
> ✅ 真相：**Rust 的 feature 是「消费时按需开启」，不是发布时预编译好多个版本！**

## 1. 底层机制说明
Crate 发布到 crates.io / 本地路径依赖时，**上传/传输的是源代码（Cargo.toml + .rs源码），不是编译好的二进制库**。
- 你作为库作者：你本地执行 `cargo build` 只是**你本机测试**，这个编译产物 `target/` 不会发给使用者。
- 使用者拉取你的源码包后，**使用者自己本地编译**，使用者在自己的 `Cargo.toml` 选择开启哪些 features，Cargo 根据选择裁剪代码、启用可选依赖。

简单一句话：
**Feature 的选择发生在【使用者编译阶段】，而不是库作者编译阶段。**

---

## 2. 角色拆分（库作者 vs 使用者）
### 🧑‍💻 角色A：你（库作者，`feature_demo`）
你只需要：
1. 写好 `Cargo.toml` 的 `[features]` + `optional = true`
2. 源码用 `#[cfg(feature = "xxx")]` 做条件编译
3. **发布源码包（crates.io / git / 源码目录）**

你本地怎么 build 只是自测，不影响下游！
你不需要编译出各种 feature 版本打包分发。

### 👤 角色B：使用者（引用你的库）
使用者在自己项目的 `Cargo.toml` 声明想要哪些特性，**使用者编译时才生效**：
```toml
# 1. 默认特性（开启 json）
feature_demo = "0.1.0"

# 2. 关闭默认，只开 async
feature_demo = { version = "0.1.0", default-features = false, features = ["async"] }

# 3. 全套功能
feature_demo = { version = "0.1.0", features = ["full"] }
```
当使用者执行 `cargo build`：
- Cargo 下载你的源码
- 根据使用者选定的 features，启用/关闭可选依赖
- `#[cfg(feature)]` 自动过滤掉不需要的代码
- 最终编译产物只包含开启特性对应的代码

---

## 3. 回答你的疑问：那库作者本地 `cargo build` 有什么用？
```bash
cargo build # 默认特性，只引入serde，无tokio
```
这一步**仅仅是作者本地自测「默认特性分支能否正常编译」**。
你此时编译出来的 `target/debug/libfeature_demo.rlib` 只是本机临时产物，**不会交付给用户**。

### 举个直观对比
1. 你本地：`cargo build` → 编译【默认feature版本】，用于你自己测试
2. 用户A：依赖不修改 → 用户本地编译【默认feature版本】
3. 用户B：`default-features=false, features=["async"]` → 用户本地编译【仅async版本】

**同一个源码包，不同使用者可以编译出完全不同的二进制代码。**

> 这和 C/C++ 静态库完全不一样！
> C库是预先编译好 `.a`，功能固定；
> Rust crate 分发源码，每个使用者就地按需编译。

---

## 4. 那什么时候需要一次性验证所有Feature？
作为库作者，你要保证所有feature组合都能编译通过，所以日常会执行：
```bash
# 验证全部特性可以正常编译
cargo check --all-features

# 验证无任何特性（关闭default）能编译
cargo check --no-default-features

# 单独测试某一组特性
cargo check --no-default-features --features async
```
CI自动化流水线一般会配置多条任务，覆盖多种 feature 组合，防止代码写错导致某条特性分支编译失败。

---

## 5. 特殊场景：如果我必须分发预编译二进制怎么办？
上面讲的是**源码分发（rust crate标准模式）**。
如果你场景特殊，需要给别人分发**已经编译好的可执行文件/预编译库**（不是源码）：
👉 这种模式**无法动态切换feature**！
预编译版本特性在编译那一刻就固定死了。
解决方案二选一：
1. 打包多个二进制：`demo-json`、`demo-async`、`demo-full`，让用户按需下载；
2. 不使用Feature，运行时动态加载插件（脱离Cargo feature体系）。

> 绝大多数Rust库（crates.io上所有包）都走【源码分发+下游自选feature】模式，不用考虑预编译。

---

## 6. 极简总结
1. Cargo Features = **编译期源码开关**，选择权在使用者，编译时生效；
2. crate 分发的是源码，不是编译产物；作者本地build只是自测，不影响下游；
3. 用户在自己 `Cargo.toml` 声明 `features = []`，自己本地编译时按需开启功能；
4. 只有分发**预编译二进制**时，feature才会被固定，无法后期修改。

## 延伸小实验
你可以把之前的 `feature_demo` 作为本地路径依赖给到consumer：
```toml
# consumer/Cargo.toml
feature_demo = { path = "../feature_demo", default-features = false, features = ["async"] }
```
进入 consumer 执行 `cargo tree`
你会发现：**哪怕feature_demo作者本地曾经build过默认版本，consumer这边依然只会引入tokio，serde消失。**
直接证明：下游的feature选择和库作者本地如何编译毫无关系。



