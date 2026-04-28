use std::{error::Error, process};

use movie_impl::read_txt_to_json;
use rfd::FileDialog;

use clap::Parser;

// use std::ops

// Box<dyn Error> 
// trait object
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    match FileDialog::new().add_filter("Text Files", &["txt"])
    .set_title("select the dvd file")
    .set_directory("./")
    .pick_file() {
        
        Some(path) => {
            println!("you selected: {:?}", path);
            let save_path = read_txt_to_json(&path)?;
            println!("save path: {save_path:?}");
            Ok(())
        }
        None => {
            println!("file not selected");
            process::exit(-1);
        }
    }
}



#[derive(Parser)]
#[command(
    version,
    about = "movie app",
    long_about = "movie information app"
)]
struct Cli {

}