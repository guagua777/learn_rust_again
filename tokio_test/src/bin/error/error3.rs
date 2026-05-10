use std::fs;
use std::net::TcpStream;
use std::io::Error;
use std::num::ParseIntError;
use std::path::Path;



fn main() {
    let stream = load_config_and_connect();
    match stream {
       Ok(stream)  => println!("Connected to server: {:?}", stream),
       Err(e) => println!("Failed to connect to server: {:?}", e),
    }
}

fn main1() {
    match connect_and_validate("127:8080") {
        Ok(stream) => {
            println!("Connected to server: {:?}", stream);
        }
        Err(e) => {
            println!("Failed to connect to server: {:?}", e);
        }
    }
}


fn connect_and_validate(address: &str) -> Result<TcpStream, Error> {
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

fn parse_address(address: &str) -> Result<(String, u16), ParseIntError> {  
    let parts: Vec<&str>= address.split(':').collect();
    let host = parts[0].to_string();
    let port = parts[1].parse::<u16>()?;
    Ok((host, port))
}

type Generic<T> = Result<T, Box<dyn std::error::Error>> ;

// 使用trait object
fn load_config_and_connect() -> Generic<TcpStream> {// Result<TcpStream, Box<dyn std::error::Error>> {// Result<TcpStream, Error> {
    let config_file = Path::new("server.txt");
    // Error
    let raw_text = fs::read_to_string(config_file)?;
    // ParseIntError
    let (host, port) = parse_address(raw_text.trim())?;
    let address = format!("{host}:{port}");
    // Error
    let stream = TcpStream::connect(&address)?;  
    Ok(stream)
}


fn load_config_and_connect2() -> anyhow::Result<TcpStream> {// Result<TcpStream, Box<dyn std::error::Error>> {// Result<TcpStream, Error> {
    let config_file = Path::new("server.txt");
    // Error
    let raw_text = fs::read_to_string(config_file)?;
    // ParseIntError
    let (host, port) = parse_address(raw_text.trim())?;
    let address = format!("{host}:{port}");
    // Error
    let stream = TcpStream::connect(&address)?;  
    Ok(stream)
}


fn anyhow_load_config_and_connect() -> anyhow::Result<TcpStream> {
    let config_file = Path::new("server.txt");
    let raw_text = fs::read_to_string(config_file)?;
    let address = raw_text.trim();

    if address.is_empty() {
        anyhow::bail!("Server address cannot be empty");
    }

    if !address.contains(':') {
        return Err(anyhow::Error::msg("Address must include port number"));
    }

    let stream: TcpStream = TcpStream::connect(address)?;
    Ok(stream)
}
