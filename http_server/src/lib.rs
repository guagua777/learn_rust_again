// 这个为什么要放到lib里面
// 把依赖放到lib里面

// 1. 什么时候创建struct？
// 2. 创建的struct的类型，是否有字段，是否使用tuple？
// 3. 什么时候创建struct的方法？
// 4. 把struct放到什么地方？

// 也可以放到util.rs文件中
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    // 为什么加这个字段？
    // 池里面为一组线程，所以要加这个字段

    // 为什么线程使用JoinHandle来表示？
    // 因为spawn的结果为JoinHandle，所以使用该类
    // 是rust的独特之处，joinhandler独占所有权
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,

    // 通信
    // sender: mpsc::Sender<Job>,
    sender: Option<mpsc::Sender<Job>>,
}

// 为什么要定义这个struct
// struct Job;

// 使用trait object
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // let mut threads = Vec::with_capacity(size);

        // for _ in 0..size {
        //     // create some threads and store them in the vector
        //     // 创建并存储线程
        // }

        // ThreadPool { threads }

        let (sender, receiver) = mpsc::channel();
        // 而且要使用mutex，确保同一时间，只有一个线程能获取
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 为了让多个线程拥有这个receiver，我们使用arc进行包装
            workers.push(Worker::new(id, receiver.clone()));
            // workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // ThreadPool { workers }

        ThreadPool {
            workers, // 只有当变量名和类型跟字段完全一样时，才可以省略
            sender: Some(sender),
        }
    }

    // 如何实现execute方法，参数为一个闭包
    // 如何确定F的约束
    // --snip--、
    // 接收一个任务（闭包），将该任务通过channel发送出去
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 把传入的闭包，封装成一个trait object，然后传递给channel
        let job = Box::new(f);

        // 此处不能用take，
        // 为什么要加.as_ref().unwrap()？
        self.sender.as_ref().unwrap().send(job).unwrap();
        // self.sender.send(f).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 有两步，
        // 1. 将threadpool中的sender置为null
        // 2. 将取出的sender drop
        drop(self.sender.take());
        // 不能使用引用，&self，必须使用self，为什么？这个&不是应用于self，而是应用于self.workers
        // drain之后，要获得所有权，所以不能使用&
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

// 为什么要定义这个worker？
// 因为要用这个代表线程，封装线程
struct Worker {
    id: usize,
    // 使用JoinHandle来代表线程，而不是使用
    thread: thread::JoinHandle<()>,
}

// impl Worker {
//     // fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         // 创建线程
//         // 里面为一个闭包，闭包的内容为空代码
//         // 这个闭包为什么要这样定义？
//         let thread = thread::spawn(move || {
//             // 这一步是为什么？
//             // receiver;
//             // 不断的从channel中获取任务，然后执行任务
//             loop {
//                 let job = receiver.lock().unwrap().recv().unwrap();

//                 println!("Worker {id} got a job; executing.");

//                 job();
//             }
//         });

//         Worker { id, thread }
//     }
// }

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

fn test() {
    //  thread::spawn(f);
    // thread::Thread
}
