use std::net::TcpStream;

fn main() {
    let address = "125:80";
    println!("begin to connect to {}", address);
    let tcpstream = TcpStream::connect(address);

    match tcpstream {
        Ok(stream) => {
            println!("Connected to {:?}", stream);
        }
        // Err(e) => {
        //     println!("Error: {:#?}", e);
        // }
        Err(e) => match e.kind() {
            std::io::ErrorKind::ConnectionRefused => {
                println!("Connection refused");
            }
            std::io::ErrorKind::NetworkUnreachable => {
                println!("Network unreachable");
            }
            _ => {
                println!("Error: {:#?}", e);
            }
        }
    }
}
