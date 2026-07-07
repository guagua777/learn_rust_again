rust 的bin文件夹介绍下，以及examples文件夹
Rust Cargo src/bin 与 examples 文件夹详解
一、src/bin 目录
1. 位置与作用
路径：src/bin/，属于 src 源码目录内。
用途：存放项目自身的多个可执行程序。
单二进制项目：只用 src/main.rs（等价 src/bin/main.rs）；
多二进制项目：把多个入口文件放入 src/bin/，每个文件都是独立程序。
2. 目录结构规则
plaintext
src/
├─ main.rs          # 主程序，二进制名 = crate 包名
└─ bin/
   ├─ cli.rs        # 编译后二进制 cli
   ├─ server.rs     # 编译后二进制 server
   └─ migrate/
      └─ db.rs      # 子文件夹生成二进制 migrate-db

规则：
每个 .rs 文件必须有 fn main()；
文件名 = 输出二进制名称；子目录会用 - 拼接目录 + 文件名；
所有 bin 文件都能直接 use crate::xxx 引用本项目 lib 库代码。
3. 常用命令
bash
运行
# 运行指定二进制
cargo run --bin cli
# 编译 release 二进制
cargo build --release --bin server
# 一次性编译所有 bin
cargo build

4. 使用场景
一个仓库维护多个配套工具：客户端 cli、后台服务、数据库迁移脚本、运维工具；
共享底层 lib 公共逻辑，不用拆多个独立项目。
5. 打包发布行为
执行 cargo build 会自动编译所有 bin；发布安装时（cargo install）会安装所有 bin 程序到系统。
二、examples 目录
1. 位置与作用
路径：项目根目录，和 src/ 同级。
用途：存放示例 Demo，写给库使用者参考，演示 API 使用方式。
目录示例：
plaintext
项目根目录
├─ src/
├─ examples/
│  ├─ quick_start.rs
│  ├─ async_demo.rs
│  └─ advanced/
│     └─ custom_config.rs
└─ Cargo.toml

2. 核心特性
每个示例文件必须包含 fn main()；
默认自动链接当前 crate，直接 use your_crate::*；
不会被 cargo build 自动编译，仅手动运行时编译；
示例可使用 [dev-dependencies] 里的依赖，不会打包进生产程序；
源码会随 crate 上传到 crates.io，方便用户查看参考。
3. 常用命令
bash
运行
# 运行单个示例
cargo run --example quick_start
# 编译示例二进制
cargo build --example async_demo
# 查看所有示例列表
cargo examples
# 子目录示例，目录名-文件名
cargo run --example advanced-custom-config

4. 使用场景
开源库配套入门、进阶演示代码；
展示复杂用法、插件、异步 / 多线程案例；
快速验证新功能，不混入业务正式程序。
三、核心对比表
表格
对比项	src/bin	examples
目录位置	src 内部	项目根目录，与 src 平级
定位	产品正式工具、业务程序	演示示例，给库用户参考
自动编译	cargo build 默认全部编译	默认不编译，需手动指定 --example
发布安装	cargo install 会安装所有 bin 到系统	仅保留源码，不会作为可执行程序安装
依赖范围	只能用普通 [dependencies]	可使用开发依赖 dev-deps
调用命令	cargo run --bin xxx	cargo run --example xxx
四、补充区分易混淆目录
tests/：集成测试，内部写 #[test] 测试函数，无 main；
benches/：基准性能测试，cargo bench 执行；
src/lib.rs：库核心代码，bin/examples/tests 全部依赖它。
五、简单最佳实践
线上可用业务程序、运维脚本、命令行工具统一放 src/bin；
所有对外演示、Demo 代码统一放入 examples；
小型单程序项目直接使用 src/main.rs，无需新建 bin 文件夹；
examples 按基础 / 异步 / 高级分文件夹，方便使用者阅读。



Rust 项目 bin/、examples/ 文件夹完整说明
基于 Cargo 标准项目结构，先理清三者核心分工：
src/bin/：项目内置可执行二进制程序（主程序、多子命令工具）
examples/：示例代码，用于演示库功能、给使用者参考
src/main.rs：单二进制项目默认入口，等价于 src/bin/main.rs
一、src/bin 文件夹
1. 作用
存放多个独立可执行文件，适合：
一个项目编译出多个独立命令行工具；
区分主程序、辅助工具、后台脚本；
替代单一 main.rs，管理多二进制。
2. 目录规则
plaintext
src/
├─ main.rs       # 等价 src/bin/main.rs，默认二进制名：项目包名
└─ bin/
   ├─ cli.rs     # 二进制 cli，编译产物 cli
   ├─ server.rs  # 二进制 server，编译产物 server
   └─ tools/     # 支持子文件夹
      └─ bench.rs# 二进制 tools-bench
规则：
每个 .rs 文件必须包含 fn main()，单独生成一个可执行文件；
文件名 = 编译后二进制名称；
子文件夹文件会拼接名称，如 bin/tools/bench.rs → tools-bench；
和根 src/main.rs 共存时，会同时产出两个程序。
3. 编译 & 运行命令
bash
运行
# 编译所有 bin 下二进制
cargo build

# 只编译/运行指定二进制
cargo run --bin cli
cargo build --bin server

# release 打包
cargo build --release --bin cli
4. 访问同项目库代码
src/bin/xxx.rs 可以直接 use crate::xxx; 引用项目 lib 内代码，和 main.rs 完全一致。
5. 使用场景
多命令工具：如同时提供 cli、daemon、migrate 迁移脚本；
单体仓库多服务，共用底层 lib 逻辑；
拆分工具脚本，不污染根目录。
二、examples 文件夹（项目根目录，和 src 同级）
1. 作用
示例演示代码，目标使用者：库的用户、开发者。
专门用来展示：库 API 怎么调用、完整使用案例、复杂功能示范。
目录层级：
plaintext
项目根目录
├─ src/
├─ examples/
│  ├─ basic_usage.rs   # 基础使用示例
│  ├─ async_demo.rs    # 异步功能示例
│  └─ advanced/
│     └─ custom_config.rs
└─ Cargo.toml
2. 核心特性
每个文件独立示例，必须带 fn main()；
自动链接当前项目库，无需额外配置，直接 use 包名::*;；
不会随 cargo build 编译，只在手动运行示例时编译；
示例之间互不干扰，每个单独编译。
3. 运行、编译示例命令
bash
运行
# 运行单个示例
cargo run --example basic_usage

# 编译示例二进制（输出 target/debug/examples/）
cargo build --example async_demo

# 列出所有示例
cargo examples
4. 子文件夹命名规则
和 bin 逻辑一致：examples/advanced/custom_config.rs → 示例名 advanced-custom-config
运行：
bash
运行
cargo run --example advanced-custom-config
5. 典型使用场景
开发开源库：给用户提供可直接复制运行的 Demo；
演示复杂 API、配置、拓展用法；
替代零散注释，提供完整可跑代码；
简易验证新功能，不混入业务二进制。
三、src/bin VS examples 核心区别（重点对比）
表格
维度	src/bin	examples
存放位置	src/bin（src 内部）	项目根目录，和 src 平级
定位	项目自身要用的程序	给库使用者看的演示代码
打包发布	会被编译、随项目发布	仅源码，发布包不包含编译产物
默认编译	cargo build 自动编译所有 bin	默认不编译，必须手动 --example
依赖	共享项目业务逻辑，生产可用	可加示例专属 dev-deps，不影响生产
用途举例	主 cli、服务端程序、迁移脚本、运维工具	入门 demo、异步示例、自定义插件案例
访问方式	cargo run --bin xxx	cargo run --example xxx
四、补充配套文件夹（顺带区分）
tests/：集成测试，文件内写测试函数 #[test]，无 main；
benches/：基准测速代码，cargo bench 执行；
src/lib.rs：库核心代码，bin/examples/tests 都依赖它。
五、最佳实践
产品业务程序、运维脚本全部放 src/bin/；
所有面向外部用户的演示代码统一放 examples/；
examples 里分基础 / 进阶目录，降低阅读门槛；
examples 中可引入仅示例使用的开发依赖，不会增大最终二进制体积；
单程序小项目直接用 src/main.rs，多程序再拆分 src/bin。