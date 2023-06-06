use serde::{Serialize, Deserialize};
use std::io;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]

pub struct Post {
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

pub fn informed_input (info: &str, input: &mut String) {
    println!("{info}");
    io::stdin().read_line(input).expect("Incorrect input");
}