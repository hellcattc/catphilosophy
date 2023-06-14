use serde::{Serialize, Deserialize};
use std::io::{self, Write, BufReader, BufRead};
use zstd::stream::{Encoder, Decoder};

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

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Quotes {
    length: usize,
    quotes: Vec<Quote>
}

impl Quotes {
    pub fn new (args: (usize, Vec<Quote>)) -> Quotes {
        Quotes {
            length: args.0,
            quotes: args.1
        }
    }
}

pub fn informed_input (info: &str, input: &mut String) {
    println!("{info}");
    io::stdin().read_line(input).expect("Incorrect input");
}

pub fn compress(data: &mut Vec<u8>, res: &mut Vec<u8>)
{
    let mut encoder: Encoder<&mut Vec<u8>> = Encoder::new(res, 0).unwrap();
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap();
}

pub fn decompress<'a>(data: &'a[u8]) -> Box<dyn BufRead + 'a> {
    let decoder = Decoder::new(data).unwrap();
    let reader = BufReader::new(decoder);
    Box::new(reader)
}