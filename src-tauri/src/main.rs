#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate reqwest;

use std::{vec};
use once_cell::sync::Lazy;
use scraper::{Html, Selector, ElementRef};
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
struct Post {
    img_url: String,
    quote_data: Quote 
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Quote {
    QuoteWithAuthor {
        quote: String,
        author: String,
    },
    NamelessQuote {
        quote: String
    }
}


pub struct ConnectionKeeper {
    client: reqwest::Client,
    photos_html: String,
    text_html: String
}

fn inner_function(item: ElementRef) -> String {
    return item.inner_html()
}

fn take_nth_element_from_vec(vec: &Vec<String>, n: usize) -> String {
    vec.clone().into_iter().nth(n).unwrap()
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
        let all_imgs = document.select(&imgs_selector).collect::<Vec<ElementRef>>();
        loop {
            let random_img = all_imgs.choose(&mut rand::thread_rng()).unwrap()
            .value().attrs().find(|(x, _)| *x == "src").expect("Wrong attrs");
            print!("{:?}", random_img);
            let img_src = random_img.1.to_string();
            if !img_src.is_empty() {
                return img_src
            }
        }
    }

    pub async fn get_text(&mut self, url: &str) -> Quote {
        if self.text_html.is_empty() {
            self.text_html = self.connect_and_get_html(url).await;
        }

        let document = Html::parse_document(&self.text_html);
        let div_selector = Selector::parse("div.su-note-inner").unwrap();
        let text_selector = Selector::parse("p").unwrap();
        let all_divs = document.select(&div_selector).collect::<Vec<ElementRef>>();

        loop {
            let random_div = all_divs.choose(&mut rand::thread_rng()).unwrap();
            let p_tags_inner = random_div.select(&text_selector).map(|x| inner_function(x)).collect::<Vec<String>>();
            println!("{:?}", p_tags_inner);
            if p_tags_inner.len() == 2 {
                if !p_tags_inner[0].is_empty() && !p_tags_inner[1].is_empty() {
                    return Quote::QuoteWithAuthor{quote: take_nth_element_from_vec(&p_tags_inner, 0), author: take_nth_element_from_vec(&p_tags_inner, 1)}
                }
            }
            else if p_tags_inner.len() == 1 {
                if !p_tags_inner[0].is_empty() {
                    return Quote::NamelessQuote{quote: take_nth_element_from_vec(&p_tags_inner, 0)}
                }
            }
        };
    }
}

pub static mut CONNECTION: Lazy<ConnectionKeeper> = Lazy::new(||ConnectionKeeper::new());
pub const CAT_PHOTOS_URL: &str = "https://yandex.ru/images/search?from=tabbar&text=%D0%BA%D0%BE%D1%82%D1%8F%D1%82%D0%B0";
pub const QUOTES_URL: &str = "https://citatnica.ru/citaty/tsitaty-velikih-filosofov-210-tsitat";

#[tauri::command]
async unsafe fn get_text_and_photos(post_count: u32) -> serde_json::Value {
    let mut posts:Vec<Post> = Vec::new();
    println!("Called backend");
    for _ in 0..post_count {
        let photo = CONNECTION.get_photos(CAT_PHOTOS_URL).await;
        let quote = CONNECTION.get_text(QUOTES_URL).await;
        posts.push(Post{img_url: photo, quote_data: quote})
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
