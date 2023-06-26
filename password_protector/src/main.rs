// lock <file> <password>
// unlock <file> <password>

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

fn fetch_args() {
    let action: Option = std::env::args().nth(1);
}

fn main() {
    println!("Hello, world!");
}
