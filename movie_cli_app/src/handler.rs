use std::io::{self, Write};
use std::{error::Error};

use crate::services::{get_logged_in_role, get_users, login_success, logout,};
use crate::models::Role;
use crate::models::Movie;
use crate::services::{read_from_json, wirte_to_json};

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("Logging in with username: {}", username);

    if let Some(user) = get_users().iter().find(|u| u.username.eq_ignore_ascii_case(username)) {
        println!("please input password:");

        // let mut input_password = String::new();
        // if io::stdin().read_line(&mut input_password).is_ok() {
        //     println!("login successful, welcome {}", user.username);
        // } else {
        //     println!("Failed to read password");
        // }

        match rpassword::read_password() {
            Ok(input_password) => {
                if input_password == user.password {
                    login_success(&user.role)?;
                    println!("login successful, welcome {}", user.username);                   
                } else {
                    println!("Incorrect password");
                }
            }
            Err(_) => println!("Failed to read password"),
        }

    } else {
        println!("User not found");
    }


    Ok(())
}


pub fn handle_logout() {
    logout();
}


pub fn handle_list() -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(role) => {
            let movies = read_from_json()?;
            println!("movie is {movies:#?}")
        },
        None => println!("You must be logged in to view the movie list"),
    }

    Ok(())
}


pub fn handle_list1() {
    println!("Here are some movies:");
    println!("1. The Shawshank Redemption");
    println!("2. The Godfather");
    println!("3. The Dark Knight");

    let movies = vec![
        "The Shawshank Redemption",
        "The Godfather",
        "The Dark Knight",
    ];

    let godfather = movies.iter()
    .filter(|i| i.contains("The"))
    .enumerate()
    .find(|(index, _)| index == &1usize)
    .map(|(_, str)| str);

    println!("Found movie: {:?}", godfather);
}

pub fn handle_add(disc: &usize, year: &str, title: &str, remark: &Option<String>) -> Result<(), Box<dyn Error>> {
    let role = get_logged_in_role()?;
    match role {
        Some(Role::Admin) => {
            let mut movies = read_from_json()?;
            let new_movie = Movie {
                disc: *disc,    
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            movies.push(new_movie);
            wirte_to_json(&movies)?;
            println!("Movie added successfully");
        },
        _ => {
            println!("You must be logged admin in to add a movie");
            return Ok(());
        }
    }
    Ok(())
}


pub fn handle_delete(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    let role = get_logged_in_role()?;
    match role {
        Some(Role::Admin) => {
            let movies = read_from_json()?;

            if let Some(m) = movies
            .iter()
            .filter(|m| m.disc == *disc)
            .enumerate()
            .find(|(i, _)| i == index)
            .map(|(_, m)| m.clone()) { // 这个地方得用clone，否则movies就被借用了，无法继续使用了
                // 注意不是iter方法
                let left_movies = movies.into_iter().filter(|m1| *m1 != m ).collect::<Vec<Movie>>();
                wirte_to_json(&left_movies)?;
                println!("Movie deleted successfully");
            } else {
                println!("No movie found with the specified disc");
            }
        },
        _ => {
            println!("You must be logged admin in to delete a movie");
            return Ok(());
        }
    }
    Ok(())
}


pub fn handle_edit(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    let role = get_logged_in_role()?;
     match role {
        Some(Role::Admin) => {
            let mut movies = read_from_json()?;

            // 获取movie的可变引用
            if let Some(m) = movies
            .iter_mut()
            .filter(|m| m.disc == *disc)
            .enumerate()
            .find(|(i, _)| i == index)
            .map(|(_, m)| m) { // 这个地方得用clone，否则movies就被借用了，无法继续使用了
                println!("enter the new disc: ");
                io::stdout().flush()?;

                // 修改moive
                



            } else {
                println!("No movie found with the specified disc");
            }
        },
        _ => {
            println!("You must be logged admin in to edit a movie");
            return Ok(());
        }
    }
    Ok(())
}
