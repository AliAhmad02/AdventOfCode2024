use std::fs;
fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let (total_price_p1, total_price_p2) = aoc12::run(&input);
    println!("Total price (problem 1): {}", total_price_p1);
    println!("Total price (problem 2): {}", total_price_p2);
}
