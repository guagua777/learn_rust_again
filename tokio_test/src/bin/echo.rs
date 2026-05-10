use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

// telnet localhost 8080

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();
        // 需要将socket移动到spawn中，所以使用move
        tokio::spawn(async move {
            let (strem_reader, mut strem_writer) = socket.split();

            let mut message = String::new();
            let mut reader = BufReader::new(strem_reader);
            loop {
                let bytes_resd = reader.read_line(&mut message).await.unwrap();
                if bytes_resd == 0 {
                    break;
                }

                strem_writer.write_all(message.as_bytes()).await.unwrap();
                message.clear();
            }
        });
    }
}
