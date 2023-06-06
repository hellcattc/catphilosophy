mod parser;

use parser::get_text_data;
use serde_json::to_string_pretty;
use std::{fs, env::current_dir, process};
use utils::informed_input;

static CONSENT_ASK: &'static str = "Type y for yes, n for no";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let current_dir = current_dir().expect("Cant get current directory");
    let formatted_current_dir = current_dir.to_str().unwrap();
    let current_path = r"\content\";
    let quotes = get_text_data().await;
    let pretty = to_string_pretty(&quotes).unwrap();
    let mut dest_dir = formatted_current_dir.to_owned() + current_path;
    println!("Parsed content will go to following path: {dest_dir}. Are you sure?");
    loop {
        let mut input = String::new();
        informed_input(CONSENT_ASK, &mut input);
        match input.trim() {
            "y" => {
                println!("Proceeding...");
                break;
            },
            "n" => {
                println!("Aborting");
                process::exit(exitcode::OK);
            },
            _ => {
                continue
            }
        };
    };
    let res = fs::create_dir(dest_dir.clone());
    if let Err(e) = res {
        println!("Error creating directory: {e}");
        dest_dir.truncate(dest_dir.len() - current_path.len() + 1);
    };
    let path = dest_dir.to_owned() + r"q.json";
    let res = fs::write(&path, pretty);
    println!("{path}");
    if res.is_err() {
        match fs::metadata(dest_dir) {
            Ok(data) => {
                println!{"{data:#?}"}
            },
            Err(e) => {
                println!{"Error getting {current_path} metadata: {e}"}
            }
        }
    };
}
