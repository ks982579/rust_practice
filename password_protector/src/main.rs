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

// Can make functions and work with those
// Or turn up complexity and work with implementations!

fn obfuscate(password: String) -> String {
    // takes ownership to delete existence
    let mut new_bytes: Vec<u8> = Vec::new();
    for byte in password.into_bytes() {
        println!("{} -> {}", &byte, String::from_utf8(vec![byte]).unwrap());
        new_bytes.push(byte + 1);
        println!("{} -> {}", &byte+1, String::from_utf8(vec![byte+1]).unwrap());
    }
    // println!("{:?}", &password.as_bytes());
    String::from_utf8(new_bytes).unwrap()
}

fn unobfuscate() {
    println!("Unblurring...")
}

fn fetch_args() {
    let action: Option<String> = std::env::args().nth(1);
}

fn main() {
    println!("Hello, world!");
    fetch_args();
}
