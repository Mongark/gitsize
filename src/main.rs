use std::env;

// default configuration parameters
// const HUMAN_READABLE: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
