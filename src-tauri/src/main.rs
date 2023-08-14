#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;
use diesel::SqliteConnection;
use tauri::State;
use vue_bStore::{establish_connection, create_book};
use std::sync::Mutex;

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
fn add_book(data: State<Data>, title: &str, author: &str, status: Option<&str>, language: &str) {
    let mut binding = data.client.lock().unwrap();
    let connection = binding.deref_mut();

    create_book(connection, title, author, status, language);
}


fn main() {
    tauri::Builder::default()
        .manage(Data::new())
        .invoke_handler(tauri::generate_handler![
            add_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
