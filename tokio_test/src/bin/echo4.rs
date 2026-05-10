use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, select, sync::broadcast, signal
};
use tokio_util::sync::CancellationToken;



// 这个程序存在一个问题：主循环没有监听取消信号。当按下 Ctrl+C 时，虽然会取消 CancellationToken 并通知所有子任务退出，但主程序的 loop 循环（第26行）会继续运行，无法退出。

// 问题分析
// 子任务（客户端连接处理）能正确响应取消信号（第54-57行）
// 但主程序的 loop { listener.accept().await ... } 没有检查取消状态
// 按下 Ctrl+C 后，子任务会退出，但主循环仍然阻塞在 listener.accept() 上
// 修复方案
// 需要在主循环中使用 select! 同时监听连接和取消信号：


// echo4.rs
// Apply
//     loop {
//         let token = token.clone();
//         let tx = tx.clone();
//         let mut rx = tx.subscribe();
//         let (mut socket, address) = select! {
//             accept = listener.accept() => accept.unwrap(),
//             _ = token.cancelled() => {
//                 println!("main loop exiting");
//                 break;
//             }
//         };
//         // 需要将socket移动到spawn中，所以使用move
//         tokio::spawn(async move {
//             let (strem_reader, mut strem_writer) = socket.split();
 
// 这个程序的问题是：主循环没有监听取消信号。

// 问题分析
// 当按下 Ctrl+C 时：

// 子任务（客户端连接处理）能正确响应取消信号并退出（第54-57行）
// 但主程序的 loop { listener.accept().await ... }（第26行）没有检查取消状态，会一直阻塞在 accept() 调用上
// 因此程序无法正常退出。

// 修复方案
// 在主循环中使用 select! 同时监听连接和取消信号。我已经生成了修复代码，请点击"Review and Accept" 应用这个修改。

// 修改后，当按下 Ctrl+C 时：

// 取消令牌被触发
// 主循环检测到取消信号后打印提示并退出
// 所有子任务也会收到取消信号并退出
// 整个程序正常终止

// telnet localhost 8080

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber);


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    let token = CancellationToken::new();
    let cancle_token = token.clone();
    tokio::spawn(async move {
        tracing::info!("ctrl c pressed");
        match signal::ctrl_c().await {
            Ok(_) => {
                tracing::warn!("ctrl c pressed");
                cancle_token.cancel();                
            }
            Err(err) => {
                tracing::error!("ctrl c error: {:?}", err);
            }
        }
    });

    loop {
        let token = token.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, address) = listener.accept().await.unwrap();
        // 需要将socket移动到spawn中，所以使用move
        tokio::spawn(async move {
            let (strem_reader, mut strem_writer) = socket.split();

            let mut message = String::new();
            let mut reader = BufReader::new(strem_reader);
            loop {
                select! {
                    result = reader.read_line(&mut message) => {
                        tracing::info!("received message : {}", message);
                        if result.unwrap() == 0 {
                            break;
                        }

                        // 发送消息到广播通道
                        tx.send((message.clone(), address)).unwrap();
                        message.clear();
                    }
                    result = rx.recv() => {
                        let (received_message, received_address)  = result.unwrap();
                        if received_address != address {
                            tracing::info!("received message from channel {}", received_message);
                            strem_writer.write_all(received_message.as_bytes()).await.unwrap();
                        }                        
                    }
                    _ = token.cancelled() => {
                        tracing::info!("task cancelled");
                        return;
                    }
                }
                
            }
        });
    }
}
