use std::{sync::{OnceLock, atomic::AtomicU32}, thread};


static LOCK: OnceLock<usize> = OnceLock::new();

fn main1() {
    assert!(LOCK.get().is_none());

    LOCK.set(10);
    assert_eq!(LOCK.get(), Some(&10));
    if let Err(value) = LOCK.set(20) {
        println!("value is ====== {}", value);
    }
    



    thread::spawn(|| {
        let lock = LOCK.get_or_init(|| 12345);
        assert_eq!(lock, &12345);
    }).join().unwrap();

    assert_eq!(LOCK.get(), Some(&12345));
   
}

// --------------------------   

use std::sync::atomic::Ordering;

static LIST: OnceList<u32> = OnceList::new();
static COUNTER: AtomicU32 = AtomicU32::new(0);

const LEN: u32 = 1000;

fn main() {

    thread::scope(|s| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            s.spawn(|| {
                while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {
                    LIST.push(i);
                }
            });
        }
    });

    for i in 0..LEN {
        assert!(LIST.contains(&i));
    }

    
}


// 结构体中的字段，使用cell，refcell，oncecell，oncelock等等
struct OnceList<T> {
    // 当前节点的数据
    data: OnceLock<T>,
    // 下一个节点
    // list: Box<OnceList<T>>,
    next: OnceLock<Box<OnceList<T>>>,
}

impl<T> OnceList<T> {

    // 为什么加const?
    const fn new() -> Self
    {
        Self {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    // 1. 生成下一个节点
    // 2. 将值赋给下一个节点
    fn push(&self, data: T) {
        if let Err(value) = self.data.set(data) {
            // 这个地方比较 精妙，好好想想
            // 如果当前data已经有值了，那就设置到下一个节点中
            // 首先初始化下一个节点，如果有值，就直接返回，没有值则初始化Box(OnceList<T>)
            let next = self.next.get_or_init(|| Box::new(OnceList::new()));
            // 递归调用push方法，将数据设置到下一个节点中
            next.push(value);
        }
    }

    fn contains(&self, data: &T) -> bool 
    // T 能够比较
        where T: PartialEq {
        self.data
        .get()
        .map(|v| v == data)
        .filter(|v| *v) // 过滤出true
        .unwrap_or_else(|| {
            self.next.get()
            .map(|next| next.contains(data))
            .unwrap_or(false)
        })
    }


    
}
