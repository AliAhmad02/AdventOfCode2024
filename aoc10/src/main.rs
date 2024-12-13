use std::fs;
use std::process;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap_or_else(|err| {
        eprintln!("Problem Reading data: {err}");
        process::exit(1);
    });

    let (total_score_p1, total_score_p2) = aoc10::total_score(&input).unwrap_or_else(|err| {
        eprintln!("Problem calculating score: {err}");
        process::exit(1);
    });

    println!("Total score problem 1: {}", total_score_p1);
    println!("Total score problem 2: {}", total_score_p2);

}
