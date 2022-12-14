use std::env;

use serde_json::Value;
use humansize::{ format_size, DECIMAL };

fn get_size_from(target: String) -> String {
    let url = "https://api.github.com/repos/".to_string()+&target;
    let client = reqwest::blocking::Client::new();

    let raw_data = client
        .get(url)
        .header("User-Agent", "getsize")
        .send()
        .expect("Failed to send message to GitHub API. Try again");

    let text_data = raw_data
        .text()
        .unwrap();

    let data: Value = serde_json::from_str(&text_data)
        .expect("Error converting data");

    let raw_size: u64 = data["size"]
        .clone()
        .as_u64()
        .expect("Erro extracting size from JSON data") * 1000;

    let formated: String = format_size(raw_size, DECIMAL);

    println!("Repository size: {}", formated);

    formated
}

#[test]
fn test_get_size_from() {
    let data = get_size_from("Mongark/gitsize".to_owned());
    assert_eq!(&data, "33 kB");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("\nGitSize - by Mongark\n   CLI tool to help you see the size of a git repository.\n   Commands:\n    gitsize <user>/<repository>\n");
        return;
    }
    let target = &args[1];

    get_size_from(target.to_owned());
}
