// Rc：多所有权、引用计数、单线程共享
// RefCell：内部可变性，不可变引用也能改内部数据
// 合起来：Rc<RefCell<T>> = 多个人拥有同一份数据，还能随时修改。

use std::rc::Rc;
use std::cell::RefCell;

// 节点别名：方便书写
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
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

// 双向链表结构体
struct DoubleList<T> {
    head: Link<T>,
    tail: Link<T>,
}


impl<T> DoubleList<T> {
    fn new() -> Self {
        DoubleList { head: None, tail: None }
    }

    // 尾部插入节点
    fn push_back(&mut self, val: T) {
        // 新建节点，包上 Rc + RefCell
        let new_node = Rc::new(RefCell::new(Node::new(val)));

        match self.tail.take() {
            // 链表非空
            Some(old_tail) => {
                // 旧尾节点的 next 指向新节点
                // 转移的是clone过后的Rc
                // clone一个新的变量Rc，然后转移这个新的变量的所有权
                // 赋值，转移所有权
                old_tail.borrow_mut().next = Some(new_node.clone());
                // 新节点的 prev 指向旧尾
                // 写的范围在该行执行完后就结束了，所以不影响后面的读
                new_node.borrow_mut().prev = Some(old_tail);
                // 更新链表尾指针
                // 转移所有权
                self.tail = Some(new_node);
            }
            // 链表为空
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }
}


fn main() {
    let mut list = DoubleList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    // 遍历打印
    let mut cur = list.head.clone();
    while let Some(node) = cur {
        println!("{}", node.borrow().val);
        cur = node.borrow().next.clone();
    }
}