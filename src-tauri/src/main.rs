mod data;

use lazy_static::lazy_static;
use serde_json::json;
use std::{string::String, cell::RefCell, borrow::BorrowMut};
use data::QUOTES_DATA;
use rand::{seq::SliceRandom, SeedableRng, rngs::StdRng};

static mut RNG_SOURCE: Option<StdRng> = None; 

#[tauri::command]
async unsafe fn get_text_and_photos(post_count: usize) -> serde_json::Value {
    let mut posts = Vec::with_capacity(post_count);
    for i in 0..post_count {
        posts[i] = QUOTES_DATA.
    };
    json!(posts)
}

fn main() {
    unsafe {tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_text_and_photos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
}
