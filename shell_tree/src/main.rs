use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn print_directory_tree(dir: &Path, prefix: &str) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error reading directory: {:?}", e);
            return;
        },
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error reading directory entry: {:?}", e);
                continue;
            },
        };

        let path = entry.path();
        let is_dir = path.is_dir();

        println!("{}{}", prefix, path.display());

        if is_dir {
            print_directory_tree(&path, &(prefix.to_owned() + "  "));
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("{:#?}", &args);

    //Check could be more Robust
    if args.len() < 2 || args.len() > 3 {
        // full path
        let name_option = std::env::current_exe(); 
        let name_buff: Option<&OsStr> = match &name_option {
            Ok(x) => x.file_name(),
            Err(e) => panic!(),
        };
        println!("{:?}", name_buff);
        let name = name_buff;

        println!("Usage: {:?} <directory> [--exclude-dir=<dir>]", &name);
        return;
    }

    let path = Path::new(&args[1]);

    //print_directory_tree(path, "");
}
