// 源头定义
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn new(s: &str) -> Self {
        Self {
            content: s.to_string(),
        }
    }

    pub fn print(&self) {
        println!("msg: {}", self.content);
    }
}

pub fn low_level_func() {
    println!("底层函数 low_level_func");
}
