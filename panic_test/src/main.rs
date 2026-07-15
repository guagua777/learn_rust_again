fn main() {
    use std::panic;

    panic::set_hook(Box::new(|panic_info| {
        println!("😱 程序出错了！");
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("错误信息: {}", s);
        }
        if let Some(location) = panic_info.location() {
            println!("在文件 {} 的第 {} 行", location.file(), location.line());
        }
    }));

    panic!("出大事了");
}
