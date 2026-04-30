为什么 multi_arc.rs中，会报错：`files_clone` does not live long enough

borrowed value does not live long enough

而cross_thread.rs中不报错


问题根源：借用 vs 所有权转移
核心差异对比
文件	关键代码	问题
multi_arc.rs	chunks = files_clone.chunks(CHUNK_SIZE)	chunks 借用了 files_clone
cross_thread.rs	vec_clone = Arc::clone(&vec)	vec_clone 是独立的 Arc 实例
详细分析
1. multi_arc.rs 的问题

rust
let files = Arc::new(files);
let files_clone = Arc::clone(&files);  // 创建了一个新的 Arc 引用
let chunks = files_clone.chunks(CHUNK_SIZE);  // ⚠️ 问题在这里！

for chunk in chunks {  // chunk 是从 chunks 迭代器获取的
    let handler = thread::spawn(move || {
        chunk.iter()...  // ❌ chunk 是借用的，生命周期不够长
    });
}
问题链：

files_clone.chunks(CHUNK_SIZE) 返回 Chunks 迭代器，它借用了 files_clone
chunk 是迭代器产生的切片（&[PathBuf]），同样借用了原始数据
thread::spawn 的闭包需要 'static 生命周期（线程可能运行任意长时间）
但 chunk 的生命周期只到 files_clone 存在的范围
编译器报错原因：chunk 是借用的数据，无法满足线程闭包的 'static 要求。

2. cross_thread.rs 为什么没问题

rust
let vec = Arc::new(vec);
let vec_clone = Arc::clone(&vec);  // 创建独立的 Arc 实例
let handler = thread::spawn(move || {
    println!("{:?}", vec_clone);  // ✅ vec_clone 被 move 进闭包，拥有所有权
});
关键：

Arc::clone(&vec) 创建的是一个新的 Arc 实例（增加引用计数）
通过 move，闭包获取了 vec_clone 的所有权
没有借用，自然没有生命周期问题

总结
情况	是否报错	原因
multi_arc.rs	❌ 报错	chunk 是借用，无法满足线程的 'static 要求
cross_thread.rs	✅ 正常	vec_clone 是独立 Arc，通过 move 转移所有权
核心原则：线程闭包中使用的数据必须是：

'static 生命周期的借用，或
通过 move 获得所有权
Arc::clone 创建的是共享所有权，但如果后续代码使用了基于 Arc 的切片/迭代器，仍然可能产生借用问题。