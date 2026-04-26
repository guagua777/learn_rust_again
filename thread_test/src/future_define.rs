use std::pin::Pin;
use std::task::{Context, Poll};

///
/// ```
/// pub enum Poll<T> {
///    Ready(T),
///    Pending,
/// }
/// 
/// ```
/// 
/// 
pub trait Future {
    // future的关联类型
    // Future 的关联类型 Output 指明了这个 future 最终会解析成什么值
    type Output;

    //接收一个特殊的 Pin 包裹的 self 引用、一个指向 Context 类型的可变引用
    // 并返回 Poll<Self::Output>
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// Pending 表示这个 future 还有工作没做完，因此调用方稍后还需要再次检查。
// Ready 则表示这个 Future 已经完成，其结果值 T 现在已经可用。
// pub enum Poll<T> {
//     Ready(T),
//     Pending,
// }
