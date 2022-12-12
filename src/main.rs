use std::env;

use serde_json::Value;

fn get_data() {
    let url = "https://api.github.com/repos/denoland/deno".to_string();
    let client = reqwest::blocking::Client::new();
    let raw_data: String = client
        .get(url)
        .header("User-Agent", "getsize")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let data: Value = serde_json::from_str(&raw_data).unwrap();

    dbg!(data);
}

fn main() {
    let _args: Vec<String> = env::args().collect();

    get_data();
}
