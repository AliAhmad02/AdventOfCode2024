use std::fs;
use std::process;
use aoc8::Grid;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap_or_else(|err| {
        eprintln!("Problem Reading data: {err}");
        process::exit(1);
    });

    let grid = Grid::build(&input).unwrap_or_else(|err| {
        eprintln!("Problem constructing grid: {err}");
        process::exit(1);
    });

    let total_antinodes = grid.total_antinodes();

    println!(
        "Total number of antinodes in unique positions: {}\nTotal number of antinodes (not just pairs) {}",
        total_antinodes.0,
        total_antinodes.1,
    );
}
