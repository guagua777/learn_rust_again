use std::cell::LazyCell;


fn main1() {
    // 而oncecell是什么样？
    // oncecell.new()
    // oncecell.get_or_init()
    // oncecell是两步，而lazycell是一步
    let lazy = LazyCell::new(init);
    println!("{}", *lazy);
    println!("{}", *lazy);
}


fn init() -> i32{
    println!("init。。。。。。。。");
    100
}


//-----------------------
use std::sync::{LazyLock, OnceLock};
use std::thread;

static NUMBER: LazyLock<i32> = LazyLock::new(|| {
    println!("init。。。。。。。。");
    200
});


fn main2() {
    let handlers: Vec<_> = (0..5).map(|_| {
        thread::spawn(|| {
            println!("{}", *NUMBER);
        })
    }).collect();

    handlers.into_iter().for_each(|handler| handler.join().unwrap());
}

//--------
static NUMBER_ONCE: OnceLock<i32> = OnceLock::new();

fn main() {
    let handlers: Vec<_> = (0..5).map(|i| {
        thread::spawn(move || {
            // NUMBER_ONCE.get_or_init(||{
            //     i as i32
            // });
            if i%2 == 0 {
                NUMBER_ONCE.set(2);
            } else {
                NUMBER_ONCE.set(1);
            }
            println!("{}", *NUMBER_ONCE.get().unwrap());
        })
    }).collect();

    handlers.into_iter().for_each(|handler| handler.join().unwrap());
}