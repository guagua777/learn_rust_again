use std::{error::Error, process};

use rfd::FileDialog;

// use std::ops

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
    match FileDialog::new().add_filter("Text Files", &["txt"])
    .set_title("select the dvd file")
    .set_directory("./")
    .pick_file() {
        
        Some(path) => {
            println!("you selected: {:?}", path);
        }
        None => {
            println!("file not selected");
            process::exit(-1);
        }
    }
}
