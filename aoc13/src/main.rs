use std::fs;
fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let total_price = aoc13::calculate_total_price(&input);
    let updated_price = aoc13::calculate_updated_price(&input);
    println!("Total price (problem 1): {}", total_price);
    println!("Total price (problem 2): {}", updated_price);
}
