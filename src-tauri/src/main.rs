#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;
use diesel::SqliteConnection;
use tauri::State;
use vue_bStore::{establish_connection, create_book, SerializedResult, get_books_interval, get_books_count};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use vue_bStore::models::Book;

struct Data {
    client: Mutex<SqliteConnection>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            client: Mutex::from(establish_connection())
        }
    }
}

#[tauri::command]
fn add_book(data: State<Data>, title: &str, author: &str, status: Option<&str>, language: &str) -> SerializedResult<()> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.deref_mut();

    create_book(connection, title, author, status, language)
}

#[derive(Serialize, Deserialize, Clone)]
struct Quote {
    author: String,
    quote: String,
}

#[derive(Serialize, Deserialize)]
struct ISBNBook {
    title: String,
    authors: Vec<(String, String)>,
}

#[tauri::command]
async fn search_book(isbn: String) -> SerializedResult<ISBNBook> {
    let response = reqwest::get(
        format!("https://openlibrary.org/api/books?bibkeys=ISBN:{}&format=json&jscmd=data", isbn
        )).await?.text().await?;
    let json: ISBNBook = serde_json::from_str(response.as_str())?;
    Ok(json)
}

#[tauri::command]
async fn get_initial_quote() -> Quote {
    let response = reqwest::get("https://dummyjson.com/quotes/random").await.ok();

    let default_quote: Quote = Quote {
        quote: "Sorry, we could not load a quote. Maybe you're offline".to_string(),
        author: "The bStore Team".to_string(),
    };

    if let Some(resp) = response {
        let req_body = resp.text().await.ok();
        if let Some(body) = req_body {
            let json: Option<Quote> = serde_json::from_str(body.as_str()).ok();

            match json {
                Some(quote) => {
                    quote
                }
                None => {
                    default_quote
                }
            }
        } else {
            return default_quote;
        }
    } else {
        return default_quote;
    }
}


#[tauri::command]
fn init_quote_generation(window: tauri::Window<tauri::Wry>) {
    std::thread::spawn(move || {
        loop {
            let req_body = reqwest::blocking::get("https://dummyjson.com/quotes/random").ok();

            let mut payload: Quote = Quote {
                quote: "Sorry, we could not load a quote. Maybe you're offline".to_string(),
                author: "The bStore Team".to_string(),
            };

            if let Some(response) = req_body {
                let req_body = response.text().ok();
                if let Some(body) = req_body {
                    let json: Option<Quote> = serde_json::from_str(body.as_str()).ok();
                    match json {
                        Some(v) => { payload = v }
                        None => {}
                    }
                }
            }
            let _result = window.emit("update_quote", payload);
            std::thread::sleep(std::time::Duration::from_secs(150));
        }
    });
}

#[tauri::command]
fn load_books_interval(data: State<Data>, limit: i64, offset: i64) -> SerializedResult<Vec<Book>> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.deref_mut();

    return get_books_interval(connection, limit, offset);
}

#[tauri::command]
fn book_count(data: State<Data>) -> SerializedResult<i64> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.deref_mut();
    get_books_count(connection)
}

fn main() {
    tauri::Builder::default()
        .manage(Data::new())
        .invoke_handler(tauri::generate_handler![
            add_book,
            init_quote_generation,
            get_initial_quote,
            load_books_interval,
            book_count,
            search_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
