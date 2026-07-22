1. src 源代码目录
    1.1 src/bin 目录
2. tests 目录
3. examples 目录

1. cargo check 
2. cargo check --all-targets
3. cargo check --package my_package

1. cargo clean
2. cargo clean --release 

1. cargo test
2. cargo test test_name
3. cargo test -- --nocapture


1. cargo doc --open
2. 



features
1. optional = true 相当于一个关键词，指示该依赖为可选依赖
2. 



https://cloud.tencent.com/developer/article/2595649

[dependencies]
# 1. crates.io 官方仓库（推荐）
serde = "1.0"                    # 最新兼容版本
tokio = "=1.25.0"               # 精确版本
regex = "1.5.*"                 # 通配符版本

# 2. Git 仓库
my_crate = { git = "https://github.com/user/my_crate.git" }
my_crate = { git = "https://github.com/user/my_crate.git", branch = "main" }
my_crate = { git = "https://github.com/user/my_crate.git", tag = "v1.0.0" }
my_crate = { git = "https://github.com/user/my_crate.git", rev = "abc123" }

# 3. 本地路径
local_crate = { path = "../local_crate" }
workspace_crate = { path = "crates/workspace_crate" }

# 4. 带特性的依赖
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "rustls-tls"], default-features = false }