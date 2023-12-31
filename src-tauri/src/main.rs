#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::ops::DerefMut;
use diesel::SqliteConnection;
use tauri::{Manager, State};
use vue_bStore::{establish_connection, create_book, SerializedResult, get_books_interval, get_books_count, FilterType, get_filtered_books_interval, get_filtered_book_count, delete_book};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use vue_bStore::models::Book;

struct Data {
    client: Mutex<Option<SqliteConnection>>
}

impl Data {
    pub fn new() -> Data {
        Data {
            client: Mutex::from(None)
        }
    }
}

#[tauri::command]
fn add_book(data: State<Data>, title: &str, author: &str, status: Option<&str>, language: &str) -> SerializedResult<()> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.as_mut().unwrap();

    create_book(connection, title, author, status, language)
}

#[derive(Serialize, Deserialize, Clone)]
struct Quote {
    author: String,
    quote: String,
}

#[derive(Serialize, Deserialize)]
struct Author {
    key: String,
}

#[derive(Serialize, Deserialize)]
struct ISBNBook {
    title: String,
    authors: Vec<Author>,
    publishers: Vec<String>,
    publish_date: String,
    number_of_pages: i64,
    covers: Vec<i64>,
}

#[tauri::command]
async fn search_book(isbn: String) -> SerializedResult<ISBNBook> {
    let response = reqwest::get(
        format!("https://openlibrary.org/isbn/{}.json", isbn
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
    let connection = binding.as_mut().unwrap();

    return get_books_interval(connection, limit, offset);
}

#[tauri::command]
fn book_count(data: State<Data>) -> SerializedResult<i64> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.as_mut().unwrap();
    get_books_count(connection)
}

#[tauri::command]
fn load_books_filtered_interval(data: State<Data>, limit: i64, offset: i64, filter: FilterType, keywords: String) -> SerializedResult<Vec<Book>> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.as_mut().unwrap();

    get_filtered_books_interval(connection, limit, offset, keywords, filter)
}

#[tauri::command]
fn filtered_book_count(data: State<Data>, filter: FilterType, keywords: String) -> SerializedResult<i64> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.as_mut().unwrap();

    get_filtered_book_count(connection, keywords, filter)
}

#[tauri::command]
fn remove_book(data: State<Data>, id: i32) -> SerializedResult<()> {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.as_mut().unwrap();

    delete_book(connection, id)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let books_db_path = app.path_resolver().resolve_resource("books").expect("No dir");

            let mut appdata_dir_path = app.path_resolver().app_data_dir().expect("No data dir");

            fs::create_dir_all(&appdata_dir_path).expect("Cannot create app path");

            appdata_dir_path.push("books");

            let mut state: State<Data> = app.state();

            *(&mut *state.client.lock().unwrap()) = Some(establish_connection( &(appdata_dir_path.as_path().display().to_string())));

            Ok(())
        })
        .manage(Data::new())
        .invoke_handler(tauri::generate_handler![
            add_book,
            init_quote_generation,
            get_initial_quote,
            load_books_interval,
            book_count,
            search_book,
            load_books_filtered_interval,
            filtered_book_count,
            remove_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
