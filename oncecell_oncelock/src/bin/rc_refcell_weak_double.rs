// Rc：强引用，计数不为 0 就不回收，互相强引用会形成循环引用 → 内存泄漏。
// Weak：弱引用，不增加引用计数，不阻止内存回收，专门用来打破循环引用。
// 双向链表天然循环：节点 A.next→B，节点 B.prev→A，全用Rc必泄漏；
// 规范写法：next 用 Rc（强引用），prev 用 Weak（弱引用）。

use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 强引用链接
type StrongLink<T> = Option<Rc<RefCell<Node<T>>>>;
// 弱引用链接
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    prev: WeakLink<T>,   // 前驱：弱引用，打破循环
    next: StrongLink<T>, // 后继：强引用，持有所有权
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            prev: None,
            next: None,
        }
    }
}

// 双向链表
struct DoubleList<T> 
where T: Display{
    head: StrongLink<T>,
    tail: StrongLink<T>,
}

impl<T> DoubleList<T> 
where T: Display {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    // 尾部插入
    fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));

        match self.tail.take() {
            Some(old_tail) => {
                // 旧尾的 next 指向新节点（强引用）
                old_tail.borrow_mut().next = Some(new_node.clone());
                // 新节点的 prev 指向旧尾（弱引用 downgrade）
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                // 更新尾指针
                self.tail = Some(new_node);
            }
            None => {
                // 空链表，头尾都指向新节点
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    // 从头到尾遍历
    fn traverse(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            print!("{} ", node.borrow().val);
            cur = node.borrow().next.clone();
        }
        println!();
    }

    // 从尾到头反向遍历（利用 Weak 升级）
    fn reverse_traverse(&self) {
        let mut cur = self.tail.clone();
        while let Some(node) = cur {
            print!("{} ", node.borrow().val);
            // 从prev的Weak升级为Rc，继续往前
            cur = node.borrow().prev.as_ref().map(|w| w.upgrade().unwrap());
        }
        println!();
    }
}


fn main() {
    let mut list = DoubleList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("正向遍历：");
    list.traverse();     // 10 20 30

    println!("反向遍历：");
    list.reverse_traverse(); // 30 20 10
}

// 单线程共享 + 内部可变性：Rc<RefCell<T>>
// 单线程共享 + 内部可变性 + 防循环泄漏：Rc<RefCell<T>> + Weak
// 多线程：Arc<Mutex<T>>