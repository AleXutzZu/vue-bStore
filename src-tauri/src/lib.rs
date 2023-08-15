pub mod models;
pub mod schema;

use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::sqlite::SqliteConnection;
use crate::models::{Book, NewBook};
use crate::schema::books;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "books";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_book(connection: &mut SqliteConnection, title: &str, author: &str, status: Option<&str>, language: &str) -> SerializedResult<()> {
    let new_book = NewBook { title, author, status, language };

    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(connection)?;
    Ok(())
}

pub fn get_books_interval(connection: &mut SqliteConnection, limit: i64, offset: i64) -> SerializedResult<Vec<Book>> {
    let result: Vec<Book> = books::table
        .select(Book::as_select())
        .order(books::id)
        .limit(limit)
        .offset(offset)
        .load(connection)?;
    Ok(result)
}

pub fn get_books_count(connection: &mut SqliteConnection) -> SerializedResult<i64> {
    let count = books::table.count().get_result(connection)?;
    Ok(count)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type SerializedResult<T> = Result<T, Error>;