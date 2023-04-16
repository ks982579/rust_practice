use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_flags(args: Vec<String>) -> Result<String, String> {
    let mut file_name = String::new();
    // Iterate through command line args
    for i in 0..args.len() {
        if args[i] == "-f" {
            if i+1 < args.len() {
                file_name = args[i+1].clone();
            }
        }
    }

    if file_name.is_empty() {
        return Err("Please provide filename using the '-f' flag. ".to_string());
    }
    return Ok(file_name);   
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    
    let file_name = match read_flags(args) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    println!("{:?}", file_name);
    let mut file = File::open("testfile.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    println!("{}", contents);
    Ok(())
}

fn _main() {
    let args = env::args(); //first arg is directory of program
    println!("{:?}", args);
}

#[test]
fn test_read_args() {
    let mut args: Vec<String> = env::args().collect();
    args.push("-f".to_string());
    args.push("example.txt".to_string());
    let flags = read_flags(args);

    assert!(flags.is_ok());
}