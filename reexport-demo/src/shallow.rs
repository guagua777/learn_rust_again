// 引入crate内部deep模块
use crate::deep;

// 必须要先use才可以 use crate::deep; 否则会报错
// 重导出到 shallow 模块下
pub use deep::Message;

// 带as 重命名导出
pub use deep::low_level_func as shallow_func;
