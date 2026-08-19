use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct BroadcastTxData {
    device_sn: String,
    group_id: String,
    counter: u64,
}

impl BroadcastTxData {
    fn get_group_id(&self) -> &str {
        &self.group_id
    }
}

/// 模拟你的场景：从 RwLock 读取数据，释放锁后继续使用
fn demo_lock_issue1() {
    // 与你的代码结构完全一致
    let shared_data: Arc<RwLock<BroadcastTxData>> = Arc::new(RwLock::new(BroadcastTxData {
        device_sn: "SN001".to_string(),
        group_id: "G001".to_string(),
        counter: 0,
    }));

    // === 场景 1：错误写法 —— 只存引用（无法编译）===
    // 假设你想这样写（注意：这段代码会编译失败）

    let self_data: &BroadcastTxData;
    {
        let sd_lock = shared_data.read().unwrap();
        self_data = &*sd_lock; // 借用 sd_lock 内部的数据
    }
    // 这里 sd_lock 已经被 drop，self_data 变成悬垂引用
    // Rust 编译器会报错：
    // "cannot borrow `*sd_lock` as mutable more than once at a time"
    // 或 "value borrowed here after temporary dropped"
}

/// 模拟你的场景：从 RwLock 读取数据，释放锁后继续使用
fn demo_lock_issue() {
    // 与你的代码结构完全一致
    let shared_data: Arc<RwLock<BroadcastTxData>> = Arc::new(RwLock::new(BroadcastTxData {
        device_sn: "SN001".to_string(),
        group_id: "G001".to_string(),
        counter: 0,
    }));

    // === 场景 1：错误写法 —— 只存引用（无法编译）===
    // 假设你想这样写（注意：这段代码会编译失败）
    //
    // let self_data: &BroadcastTxData;
    // {
    //     let sd_lock = shared_data.read().unwrap();
    //     self_data = &*sd_lock;  // 借用 sd_lock 内部的数据
    // }
    // // 这里 sd_lock 已经被 drop，self_data 变成悬垂引用
    // // Rust 编译器会报错：
    // // "cannot borrow `*sd_lock` as mutable more than once at a time"
    // // 或 "value borrowed here after temporary dropped"

    // === 场景 2：正确写法 —— clone 数据（与你的代码一致）===
    let shared_data_clone = shared_data.clone();
    // 这个地方为什么要使用Option，因为rust中没有null，一个变量要么有值，要么是Option中的None，两者统一一下，所以只能使用Option，这一点跟java是不一样的
    let mut self_data: Option<BroadcastTxData> = None;
    {
        let sd_lock = shared_data_clone.read().unwrap();
        // 这个地方为什么要clone？
        // 不clone的话，程序块结束，guard就释放了，而self_data想持有数据，所以要clone一份
        self_data = Some(sd_lock.clone()); // clone 得到独立所有权
    }
    // 此时 sd_lock 已被 drop，读锁已释放
    // 但 self_data 持有独立副本，可以安全使用
    if let Some(tx_data) = &self_data {
        println!("场景2 - group_id: {}", tx_data.get_group_id());
        println!("场景2 - counter: {}", tx_data.counter);
    }

    // === 场景 3：对比 —— 如果不提前 clone，会怎样？===
    // 假设你在持有读锁的同时去获取另一个写锁（与你的 set_device_latest_pull_at 一致）
    let cache_data: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));
    let cache_data_clone = cache_data.clone();

    // 方案 A：不 clone，直接用锁内数据（避免了提前 clone，但要嵌套持锁）
    {
        let sd_lock = shared_data.read().unwrap();
        // 这里还持有 shared_data 的读锁
        let group_id = sd_lock.get_group_id().to_string();

        // 现在想获取 cache_data 的写锁 —— 这是安全的（不同锁）
        // 但如果另一个线程也想获取 shared_data 的写锁，它会被阻塞
        // 因为我们还持有 shared_data 的读锁
        let mut cd_lock = cache_data_clone.write().unwrap();
        cd_lock.push(group_id);
        // sd_lock 和 cd_lock 都在这里被 drop
    }

    println!("场景3 - cache_data: {:?}", *cache_data.read().unwrap());

    // === 场景 4：演示并发场景下为什么提前 clone 更好 ===
    let shared_data_clone2 = shared_data.clone();
    let cache_data2: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));
    let cache_data2_clone = cache_data2.clone();

    // 提前 clone，释放 shared_data 锁后再获取 cache_data 锁
    let self_data_4: BroadcastTxData;
    {
        let sd_lock = shared_data_clone2.read().unwrap();
        self_data_4 = sd_lock.clone(); // clone 后立即释放读锁
    }
    // 此时 shared_data 的读锁已释放，其他线程可以获取写锁
    // 再获取 cache_data 的写锁，两个锁完全独立，不会互相阻塞
    let mut cd_lock2 = cache_data2_clone.write().unwrap();
    cd_lock2.push(self_data_4.get_group_id().to_string());

    println!("场景4 - cache_data: {:?}", *cache_data2.read().unwrap());
}

/// 演示多线程环境下的锁竞争问题
fn demo_contention() {
    let shared_data: Arc<RwLock<BroadcastTxData>> = Arc::new(RwLock::new(BroadcastTxData {
        device_sn: "SN002".to_string(),
        group_id: "G002".to_string(),
        counter: 0,
    }));

    // 线程1：长时间持有读锁，同时获取另一个锁
    let sd1 = shared_data.clone();
    let handle1 = thread::spawn(move || {
        let lock = sd1.read().unwrap();
        // 模拟耗时操作
        thread::sleep(Duration::from_millis(500));
        println!("线程1 在读锁内做了一些耗时操作");
        // 这里才释放锁
        drop(lock);
    });

    // 线程2：想获取写锁，会被阻塞 500ms
    let sd2 = shared_data.clone();
    let handle2 = thread::spawn(move || {
        let start = std::time::Instant::now();
        let mut lock = sd2.write().unwrap(); // 被线程1阻塞
        lock.counter += 1;
        let elapsed = start.elapsed();
        println!("线程2 获取写锁花了 {:?}（被线程1阻塞）", elapsed);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    //
    //
    println!("=== 演示锁 clone 场景 ===\n");
    demo_lock_issue();

    println!("\n=== 演示锁竞争场景 ===\n");
    demo_contention();
}
