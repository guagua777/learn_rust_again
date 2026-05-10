use thiserror::Error;
use std::fs;


#[derive(Debug, Error)]
enum BooksError {
    #[error("No books found")]
    BookNotFound,
    #[error("Too many books found")]
    TooManyBooks,

    #[error("File read failed")]
    FileReadFailed,

   
}

fn load_books() -> Result<Vec<String>, BooksError> {
    //返回的是std::io::Error，并不是BooksError，所以需要转换一下
    // let content = fs::read_to_string("books.txt")?;
    // map, 映射的是正常值
    // map_err, 映射的是错误值
    let content = fs::read_to_string("books.txt").map_err(|e| BooksError::FileReadFailed)?;

    let books: Vec<String> = content
    .lines()
    .map(|line|line.trim())
    .filter(|line| !line.is_empty())
    .map(String::from)
    .collect();

    if books.is_empty() {
        return Err(BooksError::BookNotFound);
    } else if books.len() > 10 {
        return Err(BooksError::TooManyBooks);
    }
    Ok(books)
}


// #[derive(Debug, Clone)]
// enum BooksError {
//     BookNotFound,
//     TooManyBooks,
// }

// impl std::fmt::Display for BooksError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match *self {
//             BooksError::BookNotFound => write!(f, "No books found"),
//             BooksError::TooManyBooks => write!(f, "Too many books found"),
//         }
//     }
// }


fn main() {

}