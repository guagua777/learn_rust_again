use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

// 没有加宏
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    // 创建一个运行时
    let rt  = Runtime::new()?;

    use tokio::runtime::Builder;

    // 多线程运行时
    // build runtime
    // let rt = Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .thread_name("my-custom-name")
    //     .thread_stack_size(3 * 1024 * 1024)
    //     .build()
    //     .unwrap();

    // // 单线程
    // let rt = runtime::Builder::new_current_thread()
    // .build()?;

    // 使用运行时来生成任务
    // Spawn the root task
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            // tokio::task::spawn_blocking(f)
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(0) => return,
                        Ok(n) => {
                            tokio::time::sleep(Duration::from_secs(3)).await;
                            n
                        },
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}
