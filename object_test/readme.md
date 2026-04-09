1. 创建trait对象
   1. 使用&和dyn，或者是Box等智能指针和dyn，来创建trait对象
2. 
```rust

pub struct Post {
    // trait对象，Box + dyn
    state: Option<Box<dyn State>>,
    content: String,
}

```