#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;
use diesel::SqliteConnection;
use tauri::State;
use vue_bStore::{establish_connection, create_book, SerializedResult};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

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

#[tauri::command]
async fn get_initial_quote() -> Quote {
    let body = reqwest::get("https://dummyjson.com/quotes/random").await.expect("Error whilst sending event")
        .text()
        .await.expect("Error whilst sending event");

    let payload: Quote = serde_json::from_str(body.as_str()).expect("Error whilst fetching quote");
    return payload;
}


#[tauri::command]
fn init_quote_generation(window: tauri::Window<tauri::Wry>) {
    std::thread::spawn(move || {
        loop {
            let req_body = reqwest::blocking::get("https://dummyjson.com/quotes/random").expect("Error whilst fetching quote").text().expect("Error whilst fetching quote");
            let payload: Quote = serde_json::from_str(req_body.as_str()).expect("Error whilst fetching quote");

            window.emit("update_quote", payload).expect("Error whilst sending event");
            std::thread::sleep(std::time::Duration::from_secs(150));
        }
    });
}

fn main() {
    tauri::Builder::default()
        .manage(Data::new())
        .invoke_handler(tauri::generate_handler![
            add_book,
            init_quote_generation,
            get_initial_quote
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
