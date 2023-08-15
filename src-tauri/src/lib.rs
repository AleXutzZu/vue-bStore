use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::models::{Book, NewBook};
use crate::schema::books;

pub mod models;
pub mod schema;

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

#[derive(Serialize, Deserialize)]
pub enum FilterType {
    Title, Author, Status, Language
}

pub fn get_filtered_books_interval(connection: &mut SqliteConnection, limit: i64, offset: i64, keywords: String, filter: FilterType) -> SerializedResult<Vec<Book>> {
    let statement = books::table.select(Book::as_select()).order(books::id);
    match filter {
        FilterType::Title => {
            let result = statement.filter(books::title.like(format!("%{}%", keywords))).limit(limit).offset(offset).load(connection)?;
            Ok(result)
        }
        FilterType::Author => {
            let result = statement.filter(books::author.like(format!("%{}%", keywords))).limit(limit).offset(offset).load(connection)?;
            Ok(result)
        }
        FilterType::Status => {
            let result = statement.filter(books::status.like(format!("%{}%", keywords))).limit(limit).offset(offset).load(connection)?;
            Ok(result)
        }
        FilterType::Language => {
            let result = statement.filter(books::language.like(format!("%{}%", keywords))).limit(limit).offset(offset).load(connection)?;
            Ok(result)
        }
    }
}

pub fn get_filtered_book_count(connection: &mut SqliteConnection, keywords: String, filter: FilterType) -> SerializedResult<i64> {
    let statement = books::table.order(books::id);
    match filter {
        FilterType::Title => {
            let result = statement.filter(books::title.like(format!("%{}%", keywords))).count().get_result(connection)?;
            Ok(result)
        }
        FilterType::Author => {
            let result = statement.filter(books::author.like(format!("%{}%", keywords))).count().get_result(connection)?;
            Ok(result)
        }
        FilterType::Status => {
            let result = statement.filter(books::status.like(format!("%{}%", keywords))).count().get_result(connection)?;
            Ok(result)
        }
        FilterType::Language => {
            let result = statement.filter(books::language.like(format!("%{}%", keywords))).count().get_result(connection)?;
            Ok(result)
        }
    }
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