use std::net::TcpStream;
use std::io::Error;


fn main() {
    match try_connect_to_server("127:8080") {
        Ok(stream) => {
            println!("Connected to server: {:?}", stream);
        }
        Err(e) => {
            println!("Failed to connect to server: {:?}", e);
        }
    }
}


fn try_connect_to_server(address: &str) -> Result<TcpStream, Error> {
    TcpStream::connect(address)
}