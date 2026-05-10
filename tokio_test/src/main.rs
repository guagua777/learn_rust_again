use tokio::runtime;


async fn hi() {
    println!("hi tokio");
        
}


// fn main() {
//     let rt = runtime::Builder::new_current_thread()
//     .enable_all()
//     .build()
//     .unwrap();

//     rt.block_on(hi());
// }


// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     hi().await;
// }


fn main() {
    let rt = runtime::Builder::new_multi_thread()
    // 设置参数
    .worker_threads(10)
    .thread_stack_size(5*1024*1024)
    .event_interval(20)
    .max_blocking_threads(256)

    .enable_all()
    .build()
    .unwrap();

    rt.block_on(hi());
}