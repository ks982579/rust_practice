// lock <file> <password>
// unlock <file> <password>

// use std::prelude::*;

// Action should be supplied by user
#[derive(Debug)]
enum Action {
    Lock,
    Unlock,
    Help,
}

struct Instructions {
    action: Action,
    path: std::path::PathBuf,
    password: String,
}

struct Limits {
    upper: i16,
    lower: i16,
}

fn fetch_args() {
    let action: Option<String> = std::env::args().nth(1);
}

fn main() {
    println!("Hello, world!");
    fetch_args();
}
