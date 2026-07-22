pub mod deep;
pub mod shallow;

// 关键：继续向上搬运到 crate 根！
pub use deep::{Message, low_level_func};

// 也可以导出shallow里重命名后的符号
pub use shallow::shallow_func;
