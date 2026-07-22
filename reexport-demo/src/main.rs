use reexport_demo::{Message, low_level_func, shallow_func};

fn main() {
    // 方式1：通过顶层reexport直接使用
    let m = Message::new("hello reexport");
    m.print();
    low_level_func();
    shallow_func();

    // 方式2：走原始深层路径，完全等价，同一个类型
    use reexport_demo::deep::inner::Message as RawMessage;
    let _raw: RawMessage = Message::new("same type");
    // ✅ 编译通过，证明是同一个类型，不是新类型

    // 方式3：走中间模块路径
    use reexport_demo::deep::Message as MidMessage;
    let _mid: MidMessage = Message::new("same");
}
