use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, select, sync::broadcast
};

// telnet localhost 8080

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    loop {
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
                            strem_writer.write_all(received_message.as_bytes()).await.unwrap();
                        }                        
                    }
                }
                
            }
        });
    }
}
