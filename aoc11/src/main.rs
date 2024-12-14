fn main() {
    let input = "0 7 6618216 26481 885 42 202642 8791";

    let n_blinks1 = 25;
    let n_blinks2 = 75;

    let stones_after_blinks1 = aoc11::num_stones_after_blinks(input, n_blinks1);
    let stones_after_blinks2 = aoc11::num_stones_after_blinks(input, n_blinks2);

    println!(
        "Number of stones after {} blinks: {}",
        n_blinks1, stones_after_blinks1
    );
    println!(
        "Number of stones after {} blinks: {}",
        n_blinks2, stones_after_blinks2
    );
}
