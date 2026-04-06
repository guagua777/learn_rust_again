use std::io::Error;
use std::io::Read;
use std::num::ParseIntError;
use std::fs::File;


#[derive(Debug)]
pub enum MyError {
    Io(Error),
    ParseInt(ParseIntError),
    Other(String),
}

impl From<Error> for MyError {
    // 将 std::io::Error 转换为 MyError
    // io类型的Error，可以通过?操作符自动转换为MyError
    fn from(err: Error) -> Self {
        Self::Io(err)
    }    
}


impl From<ParseIntError> for MyError {
    // 将 ParseIntError 转换为 MyError
    // ParseIntError，可以通过?操作符自动转换为MyError  
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }    
}



fn read_username_from_file() -> Result<String, MyError> {
    let mut username = String::new();
    let file = File::open("username.txt")?.read_to_string(&mut username)?;
    let num: i32 = "55".parse()?;
    Ok(username)
}

fn main() {
    println!("Hello, world!");
}
