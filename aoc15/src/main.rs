use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    aoc15::run(&input);
}