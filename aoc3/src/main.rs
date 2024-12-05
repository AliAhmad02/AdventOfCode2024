use aoc3;
use std::process;

fn main() {
    let file_path = "data.txt";
    if let Err(e) = aoc3::run(file_path) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
