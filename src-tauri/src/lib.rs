pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::models::{Book, NewBook};

pub fn establish_connection() -> SqliteConnection {
    let database_url = "books";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_book(connection: &mut SqliteConnection, title: &str, author: &str, status: Option<&str>, language: &str) {
    use crate::schema::books;

    let new_book = NewBook { title, author, status, language };

    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(connection)
        .expect("Error saving book");
}

pub fn get_books(connection: &mut SqliteConnection) -> Vec<Book> {
    use self::schema::books::dsl::*;

    books.select(Book::as_select()).load(connection).expect("Error loading posts")
}