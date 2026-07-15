use core::num;
use std::thread;


fn main() {
    // vec -> box
    let mut numbers: Box<[usize]> = Vec::from_iter(0..=1000).into_boxed_slice();
    // box -> &[usize]切片
    // let numbers: &mut [usize] = &mut numbers[..];
    
    // 变为'static 生命周期
     let numbers: &mut [usize] = Box::leak(numbers);

    let t = thread::spawn(|| {
        // ERROR on numbers lifetime
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
}
