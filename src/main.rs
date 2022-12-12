use std::env;

use serde_json::Value;

fn get_data(target: String) {
    let url = "https://api.github.com/repos/".to_string()+&target;
    let client = reqwest::blocking::Client::new();
    let raw_data: String = client
        .get(url)
        .header("User-Agent", "getsize")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let data: Value = serde_json::from_str(&raw_data).unwrap();
    let size = &data["size"];

    println!("Size: {}", &size);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = &args[1];

    get_data(target.to_owned());
}
