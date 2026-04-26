use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {

    // Stream 的关联类型 Item 指明了这个 stream 最终会解析成什么值
    type Item;

    // 类似于poll函数，第一个参数为pin后的self，第二个参数为Context的可变引用，返回值为Poll<Option<Self::Item>>
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;

    // 参考future的方法
    // fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
