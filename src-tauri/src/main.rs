#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate reqwest;

use utils::Quote;
use serde_json::json;
use std::string::String;

#[tauri::command]
async unsafe fn get_text_and_photos(post_count: usize) -> serde_json::Value {
    let mut posts = Vec::new();
    for i in 0..post_count {
        posts[i] = Quote::NamelessQuote { quote: (String::from("Hello")) }
    }
    json!(posts)
}

fn main() {
    unsafe {tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_text_and_photos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
}
