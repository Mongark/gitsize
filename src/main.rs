use std::env;

fn get_data() {
    let url = "https://api.github.com/repos/denoland/deno".to_string();
    let client = reqwest::blocking::Client::new();
    let data: _ = client
        .get(url)
        .header("User-Agent", "getsize")
        .send()
        .unwrap()
        .text()
        .unwrap();

    dbg!(data);
}

fn main() {
    let _args: Vec<String> = env::args().collect();

    get_data();
}
