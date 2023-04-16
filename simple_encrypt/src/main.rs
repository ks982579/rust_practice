use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("testfile.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    println!("{}", contents);
}
