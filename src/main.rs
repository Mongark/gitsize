use std::env;

fn get_metadata() {
    let url = "https://api.github.com/repos/denoland/deno".to_string();

    let data = reqwest::blocking::get(url).unwrap().text();

    dbg!(data);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    get_metadata();
}
