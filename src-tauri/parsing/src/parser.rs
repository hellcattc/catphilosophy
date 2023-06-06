use reqwest::get;
use utils::Quote;
use scraper::{Html, Selector, ElementRef};
use once_cell::sync::Lazy;

const QUOTES_URL: &str = "https://citatnica.ru/citaty/tsitaty-velikih-filosofov-210-tsitat";
static DIV_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("div.su-note-inner").unwrap());
static TEXT_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("p").unwrap());

pub async fn get_text_data() -> Vec<Quote>{
    let response = get(QUOTES_URL).await.expect("Connection problems").text().await.expect("Error converting bytes to text");
    let document = Html::parse_document(&response);
    let all_divs = document.select(&DIV_SELECTOR).collect::<Vec<ElementRef>>();
    let mut quotes: Vec<Quote> = Default::default();
    for div in all_divs.into_iter() {
        let p_tags_inner = div.select(&TEXT_SELECTOR)
            .map(|x| return x.inner_html())
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>();
        match p_tags_inner.len() {
            0 => {
                continue;
            },
            1 => {
                quotes.push(Quote::NamelessQuote { quote: (p_tags_inner[0].clone().replace("&nbsp", "")) })
            },
            2 => {
                quotes.push(Quote::QuoteWithAuthor { quote: (p_tags_inner[0].clone().replace("&nbsp", "")), author: (p_tags_inner[1].clone()) })
            },
            _ => {
                continue;
            }
        }
    }
    return quotes
}