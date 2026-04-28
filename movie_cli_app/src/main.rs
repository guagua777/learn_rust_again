use clap::Command;
use clap::Parser;
use clap::Subcommand;
use movie_cli_app::handler::handle_login;
use movie_cli_app::handler::handle_logout;
use movie_cli_app::handler::{handle_list, handle_add, handle_delete, handle_edit};

use std::error::Error;


#[derive(Parser)]
#[command(
    version,
    about = "movie app",
    long_about = "movie information app"
)]
// struct Cli {
//     name: String,
//     #[arg(short, long)]
//     age: u32,
// }


struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// Login to the app
    Login {
        /// Username to login with
        #[arg(short, long)]
        username: String
    },

    /// Logout from the app
    Logout,


    /// List all movies
    List,


    /// Add a movie
    Add {
        /// Disc number of the movie
        #[arg(short, long)]        
        disc: usize,

        /// Year of the movie
        #[arg(short, long)]
        year: String,

        /// Year of the movie
        #[arg(short, long)]
        title: String,

        /// Remark of the movie
        #[arg(short, long)]
        remark: Option<String>,
    },

    /// Delete a movie
    Delete {
        /// Disc number of the movie to delete
        #[arg(short, long)]
        disc: usize,

        /// Index of the movie to delete
        #[arg(short, long)]
        index: usize,
    },


    /// Edit a movie
    Edit {
        /// Disc number of the movie to edit
        #[arg(short, long)]
        disc: usize,

        /// Index of the movie to edit
        #[arg(short, long)]
        index: usize,
    },
    
} 



fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => {
            handle_login(username)?
        },
        Some(Commands::Logout) => {
            println!("Logging out...");
            // Here you would add logic to clear the session or perform any necessary cleanup
            handle_logout()
        },
        Some(Commands::List) => {
            println!("Listing all movies...");
            // Here you would add logic to list movies, e.g., read from a file or database
            handle_list()?
        },

        Some(Commands::Add { disc, year, title, remark }) => {  
            println!("Adding a new movie...");
            // Here you would add logic to add a new movie, e.g., write to a file or database
            println!("Disc: {}, Year: {}, Title: {}, Remark: {}",
                disc,
                year,
                title,
                remark.as_deref().unwrap_or("None")
            );
            handle_add(disc, year, title, remark)?
        },

        Some(Commands::Delete { disc, index }) => {
            println!("Deleting movie with disc: {} and index: {}", disc, index);
            // Here you would add logic to delete a movie, e.g., remove from a file or database
            handle_delete(disc, index)?;
        },

        Some(Commands::Edit { disc, index }) => {
            println!("Editing movie with disc: {} and index: {}", disc, index);
            handle_edit(disc, index)?;
        },

        _ => println!("No command provided"),
    }

    Ok(())
}