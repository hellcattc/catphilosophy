#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate reqwest;

use std::{vec};
use once_cell::sync::Lazy;
use scraper::{Html, Selector, ElementRef};
use rand::seq::SliceRandom;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
#[derive(Debug)]
struct Post {
    img_url: String,
    quote_text: String
}

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

    pub async fn get_photos(&mut self, url: &str) -> Option<String> {
        if self.photos_html.is_empty() {
            self.photos_html = self.connect_and_get_html(url).await;
        }
        let document = Html::parse_document(&self.photos_html);
        let imgs_selector = Selector::parse("img").unwrap();
        let img = document.select(&imgs_selector).collect::<Vec<ElementRef>>().choose(&mut rand::thread_rng()).unwrap()
        .value().attrs().find(|(x, _)| *x == "src").expect("Wrong attrs");
        print!("{:?}", img);
        let img_src = img.1.to_string();
        if img_src.is_empty() {
            return None
        } else {
            return Some(img_src)
        }
    }

    pub async fn get_text(&mut self, url: &str) -> Option<String> {
        if self.text_html.is_empty() {
            self.text_html = self.connect_and_get_html(url).await;
        }
        let document = Html::parse_document(&self.text_html);
        let div_selector = Selector::parse("div.su-note").unwrap();
        let text_selector = Selector::parse("p").unwrap();
        let all_divs = document.select(&div_selector).collect::<Vec<ElementRef>>();
        let random_div = all_divs.choose(&mut rand::thread_rng()).unwrap();
        let p_inner = random_div.select(&text_selector).collect::<Vec<ElementRef>>()[0].inner_html();
        if p_inner.is_empty() {
            return None
        } else {
            return Some(p_inner)
        }
    }
}

pub static mut CONNECTION: Lazy<ConnectionKeeper> = Lazy::new(||ConnectionKeeper::new());
pub const CAT_PHOTOS_URL: &str = "https://yandex.ru/images/search?from=tabbar&text=%D0%BA%D0%BE%D1%82%D1%8F%D1%82%D0%B0";
pub const QUOTES_URL: &str = "https://citatnica.ru/citaty/tsitaty-velikih-filosofov-210-tsitat";

#[tauri::command]
async unsafe fn get_text_and_photos(post_count: u32) -> serde_json::Value {
    let mut posts:Vec<Post> = Vec::new();
    for _ in 0..post_count {
        let mut photo = CONNECTION.get_photos(CAT_PHOTOS_URL).await;
        let mut quote = CONNECTION.get_text(QUOTES_URL).await;
        while quote.is_none() {
            quote = CONNECTION.get_text(QUOTES_URL).await;
            println!("Relaunched quote fetch")
        }
        while photo.is_none() {
            photo = CONNECTION.get_photos(CAT_PHOTOS_URL).await;
            println!("Relaunched photo fetch")
        }
        posts.push(Post{img_url: photo.unwrap(), quote_text: quote.unwrap()})
    }
    println!("Here are posts");
    println!("{:?}", posts);
    json!(posts)
}

fn main() {
    unsafe {tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_text_and_photos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
}
