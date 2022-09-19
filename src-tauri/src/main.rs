#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate reqwest;

use std::vec;

use once_cell::sync::Lazy;
use scraper::{Html, Selector, ElementRef};
use rand::seq::SliceRandom;

pub struct ConnectionKeeper {
    client: reqwest::Client,
    photos_html: String,
    text_html: String
}

impl ConnectionKeeper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            photos_html: "".to_string(),
            text_html: "".to_string(),
        }
    }
    async fn connect_and_get_html(&self, url: &str) -> String  {
        let response = self.client.get(url).send().await.expect("Wrong response");
        assert!(response.status().is_success());
        return response.text().await.unwrap()
    }

    pub async fn get_photos(&mut self, url: &str) -> String {
        if self.photos_html.is_empty() {
            self.photos_html = self.connect_and_get_html(url).await;
        }
        let document = Html::parse_document(&self.photos_html);
        let imgs_selector = Selector::parse("img").unwrap();
        let img = document.select(&imgs_selector).collect::<Vec<ElementRef>>().choose(&mut rand::thread_rng()).unwrap()
        .value().attrs().find(|(x, _)| *x == "src").expect("Wrong attrs");
        print!("{:?}", img);
        img.1.to_string()

    }
}

pub static mut CONNECTION: Lazy<ConnectionKeeper> = Lazy::new(||ConnectionKeeper::new());
pub const CAT_PHOTOS_URL: &str = "https://fonwall.ru/";

#[tauri::command]
async unsafe fn get_cat_photo() -> String {
    CONNECTION.get_photos(CAT_PHOTOS_URL).await
}

fn main() {
    unsafe {tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cat_photo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
}
