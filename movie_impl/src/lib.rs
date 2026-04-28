use std::{error::Error, fs, path::PathBuf};

use regex::{Regex, bytes::CaptureMatches};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};


pub fn read_txt_to_json(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let mut disc_no = 0u32;

    let disc_regex = Regex::new(r"^(\d+)\.$").unwrap();
    let movie_regex = Regex::new(r"^(\d{4})(.*?)(（儿童）)?$")?;
    let mut movies = Vec::new();
    for line in content.lines().map(str::trim).filter(|line| !line.is_empty()) {
        // println!("{line}");
        if let Some(no) = disc_number(line, &disc_regex) {
            // println!("{no}");
            disc_no = no;
        } else {
            if let Some(movie) = parse_movie(disc_no, line, &movie_regex) {
                movies.push(movie);
            }
        }
    }
    save_to_json(&movies)
}


fn save_to_json(movies: &Vec<Movie>) -> Result<PathBuf, Box<dyn Error>> {
    let json = serde_json::to_string_pretty(movies)?;
    let path = FileDialog::new().add_filter("JSON", &["json"])
    .set_title("save the json file")    
    .set_directory(r"./")
    .save_file().ok_or_else(|| "no select save file".to_string())?;

    fs::write(&path, json)?;
    Ok(path)
}



fn parse_movie(disc_no: u32, line: &str, re: &Regex) -> Option<Movie> {
    re.captures(line).map(|caps| {
        // println!("Caps: {caps:#?}");
        Movie {
        disc: disc_no,
        year: caps.get(1).unwrap().as_str().trim().to_string(),
        title: caps.get(2).unwrap().as_str().trim().to_string(),
        remark: if let Some(remark) = caps.get(3) {
            Some(remark.as_str().trim().to_string())
        } else {
            None
        },
    }
    })
     
   
}



fn disc_number(line: &str, re: &Regex) -> Option<u32> {
    // if let Some(caps) = re.captures(line) {
    //     // println!("Caps: {caps:#?}");
    //     Some(caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
    // } else {
    //     None
    // }
    re.captures(line).map(|caps| caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
   
}


#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    disc: u32, 
    year: String, 
    title: String, 
    remark: Option<String>,
}