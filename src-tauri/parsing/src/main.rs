mod parser;

use parser::get_text_data;
use serde_json::to_writer;
use std::{fs, env::current_dir, process};
use utils::{informed_input, compress};

static CONSENT_ASK: &'static str = "Type y for yes, n for no";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let current_dir = current_dir().expect("Cant get current directory");
    let formatted_current_dir = current_dir.to_str().unwrap();
    let current_path = r"\content\";
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
    let mut writer: Vec<u8> = Vec::new();
    let mut buf: Vec<u8> = Vec::new();
    to_writer(&mut writer, &get_text_data().await.1).unwrap();
    compress(&mut writer, &mut buf);
    if let Err(e) = fs::create_dir(dest_dir.clone()) {
        if !(e.raw_os_error().unwrap() == 183) {
            println!("Error creating directory: {}", e.kind());
            dest_dir.truncate(dest_dir.len() - current_path.len() + 1);
        }
    };
    let path = dest_dir.to_owned() + r"q.json.zst";
    if let Err(e) = fs::write(&path, buf) { 
        println!("{e}");
        match fs::metadata(dest_dir) {
            Ok(data) => {
                println!{"{data:#?}"};
            },
            Err(e) => {
                println!{"Error getting {current_path} metadata: {e}"};
            }
        }
    };
}
