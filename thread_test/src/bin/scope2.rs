use std::thread;

fn sum(arr: &[i32]) -> i32 {
    if arr.len() <= 2 {
        arr.iter().sum()
    } else {
        let mid = arr.len() / 2;
        let (left, right) = arr.split_at(mid);
        thread::scope(|s| {
            let h = s.spawn(|| sum(left));
            sum(right) + h.join().unwrap()
        })
    }
}

fn main() {
    let nums = [1,2,3,4,5,6,7,8];
    println!("{}", sum(&nums));
}