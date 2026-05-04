use std::sync::Mutex;
use std::thread;


static NUMBERS: Mutex<Vec<i32>> = Mutex::new(Vec::new());

#[test]
fn main1() {
    let mut handlers = Vec::new();

    for _ in 0..20 {
        let h = thread::spawn(move || {
            let mut numbers = NUMBERS.lock().unwrap();
            // 因为Mutex实现了DerefMut，所以可以使用push方法
            // impl<T: ?Sized> DerefMut for MutexGuard<'_, T> 
            numbers.push(100);
        });
        handlers.push(h);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());


    let numbers = NUMBERS.lock().unwrap();
    println!("{numbers:#?}");
}