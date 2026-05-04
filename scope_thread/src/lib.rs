use std::thread;

#[test]
fn scope_thread() {
    const CHUNK_SIZE: usize = 10;
    let number: Vec<u32> = (1..10000).collect();
    let chunks = number.chunks(CHUNK_SIZE);

    // thread::scope 的设计目的是确保：
    // 作用域内的线程必定在作用域结束前完成
    // 因此可以安全地引用作用域外部的数据（如 chunk）
    // 但这也意味着 ScopedJoinHandle 不能逃逸出 scope——这正是你的代码报错的原因。
    // let mut handlers = Vec::new();
    // for chunk in chunks {
    //     thread::scope(|s| {
    //         let h = s.spawn(|| chunk.iter().sum::<u32>());
    //         handlers.push(h);
    //     })
    // }


    let sum = thread::scope(|s| {
        let mut handlers = Vec::new();
        for chunk in chunks {
            let h = s.spawn(|| chunk.iter().sum::<u32>());
            handlers.push(h);
        }
        handlers.into_iter().map(|h| h.join().unwrap())
        .sum::<u32>()
    });

    println!("sum: {}", sum);

}