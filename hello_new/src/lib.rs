// 语义：
// 线程池接受任务 ---> 使用channel将任务发送给worker ---> worker执行任务

use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread};

/// 系统1，对外的系统：
/// 输入：闭包
/// 输出：无
/// 按钮：execute
/// 
/// 系统2，内部系统：
/// 输入：数量
/// 输出：Worker集合 + channel
/// 按钮：new

// 里面有多个启动的线程
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// 1. 创建线程池
/// 2. 发送任务给worker
impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {

        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // recevier要发送给多个worker，所以要克隆
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }

    }

    pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static {

        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
        
    

}



struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}


impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // 创建线程
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    },
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }

            };
        });
        
        Worker { id, thread }
    }

    
}