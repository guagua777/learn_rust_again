use std::{sync::atomic::AtomicUsize, time::Instant};
use rand::random_range;
use std::thread;


fn main() {
    let start = Instant::now();

    // 库房数量
    let section_count = random_range(10..=20);
    let mut sections = Vec::new();

    // 每个产品的数量
    // index为产品的索引
    let mut actual = [0; 5];

    for _ in 0..section_count {
        // 每个仓库中定义一个 Section，包含 5 个产品的数量
        let mut section = Section([0; 5]);
        for (i, p) in section.0.iter_mut().enumerate() {
            // 给数组赋值，并且累加到 actual 中
            *p = random_range(0..=1_000_000);
            actual[i] += *p;
        }
        sections.push(section);
    }


    println!("Actual: {actual:#?}");
    // println!("sections: {sections:#?}");

    // 为什么要使用[AtomicUsize; 5]类型
    // 因为要并发的给每个产品，进行计数
    let counted: [AtomicUsize; 5] = Default::default();

    thread::scope(|s| {
        // 遍历仓库
        for section in &sections {
            s.spawn(|| {
                for (i, p) in section.0.iter().enumerate() {
                    // 遍历每一个数量，盘点
                    for _ in 0..*p {
                        // let a = counted[i];
                        counted[i].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }

                    // counted[i].fetch_add(*p, std::sync::atomic::Ordering::SeqCst);  
                    // counted[i].fetch_add(*p, std::sync::atomic::Ordering::Relaxed); 

                }
            });
        }
    });

    println!("Counted: {counted:#?}");

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

// 每个仓库中的产品数量
#[derive(Debug)]
struct Section ([usize; 5]);