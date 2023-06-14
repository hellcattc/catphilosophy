use serde_json::from_reader;
use utils::{Quote, decompress};


const COMPRESSED_DATA: &[u8; 9165] = include_bytes!("../../content/q.json.zst");

pub static QUOTES_DATA: Vec<Quote> = {
    let decompressed = decompress(COMPRESSED_DATA);
    from_reader(decompressed).unwrap()
};
