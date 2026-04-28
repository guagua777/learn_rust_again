use unicode_width::UnicodeWidthStr;

use crate::models::User;    
use crate::models::Role;
use std::fs;
use std::error::Error;
use crate::models::Movie;
use std::io;


pub fn get_users() -> Vec<User> {
    vec![
        User { username: "Admin".to_string(), password: "admin".to_string(), role: Role::Admin },
        User { username: "Bob".to_string(), password: "bob".to_string(), role: Role::User },
        User { username: "Charlie".to_string(), password: "charlie".to_string(), role: Role::User },
    ]
}


pub fn login_success(role: &Role) -> Result<(), Box<dyn Error>> {
    fs::write(".session", role.to_string())?;
    Ok(())
}

pub fn get_logged_in_role() -> Result<Option<Role>, Box<dyn Error>> {
    let role = fs::read_to_string(".session")?;
    match role.as_str() {
        "Admin" => Ok(Some(Role::Admin)),
        "User" => Ok(Some(Role::User)),
        _ => Ok(None),
    }
}

pub fn logout() {
    fs::remove_file(".session").unwrap_or_else(|_| {
        println!("No session file found to remove");
    });
}


pub fn read_from_json() -> Result<Vec<Movie>, Box<dyn Error>> {
    let file = fs::File::open("movie.json")?;
    let reader = io::BufReader::new(file);
    let movies: Vec<Movie> = serde_json::from_reader(reader)?;
    Ok(movies)
}


pub fn list_movies(movies: &[Movie]) {
    movies.iter().for_each(|movie| {
        println!("Disc: {}, Year: {}, Title: {}, Remark: {}",
            movie.disc,
            movie.year,
            movie.title,
            movie.remark.as_deref().unwrap_or("None")
        );
    });
}

pub fn pad_display_width(s: &str, width: usize) -> String {
    let width = UnicodeWidthStr::width(s);
    format!("{}{}", s, " ".repeat(width.saturating_sub(width)))

}


pub fn wirte_to_json(movies: &[Movie]) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create("movie.json")?;
    serde_json::to_writer_pretty(file, movies)?;
    Ok(())
}