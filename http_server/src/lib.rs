// 这个为什么要放到lib里面
// 把依赖放到lib里面


// 1. 什么时候创建struct？
// 2. 创建的struct的类型，是否有字段，是否使用tuple？
// 3. 什么时候创建struct的方法？
// 4. 把struct放到什么地方？


// 也可以放到util.rs文件中

use std::thread;
pub struct ThreadPool {
    // 为什么加这个字段？
    // 池里面为一组线程，所以要加这个字段

    // 为什么线程使用JoinHandle来表示？
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }


    // 如何实现execute方法，参数为一个闭包
    // 如何确定F的约束
    // --snip--
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

       
    }
}


fn test() {
    //  thread::spawn(f);
    // thread::Thread
}