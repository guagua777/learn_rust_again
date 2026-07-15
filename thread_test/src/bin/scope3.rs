use std::thread;
use std::time::{Duration, Instant};

// 批量计算任务：模拟耗时运算
fn heavy_calc(n: u64) -> u64 {
    let mut res = n;
    for _ in 0..5_000_000 {
        res = res * 3 % 1_000_007;
    }
    res
}

fn main() {
    let input_data: Vec<u64> = (1..=8).collect();
    println!("原始数据: {:?}", input_data);

    // ---------------- 串行版本 ----------------
    let start_serial = Instant::now();
    let mut result_serial = Vec::new();
    for &num in &input_data {
        result_serial.push(heavy_calc(num));
    }
    let cost_serial = start_serial.elapsed();
    println!("\n【串行结果】{:?}", result_serial);
    println!("串行耗时: {:?}", cost_serial);

    // ---------------- thread::scope 并行版本 ----------------
    let start_parallel = Instant::now();
    let mut result_parallel = vec![0; input_data.len()];

    thread::scope(|scope| {
        // 逐个创建子线程执行计算
        let mut handles = Vec::new();
        for (idx, &num) in input_data.iter().enumerate() {
            let handle = scope.spawn(move || {
                heavy_calc(num)
            });
            handles.push((idx, handle));
        }

        // 收集子线程结果
        for (idx, handle) in handles {
            result_parallel[idx] = handle.join().unwrap();
        }
    });

    let cost_parallel = start_parallel.elapsed();
    println!("\n【scope并行结果】{:?}", result_parallel);
    println!("并行耗时: {:?}", cost_parallel);
}