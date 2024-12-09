use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let formatted = format_input(&input);
    let checksum = calculate_checksum(formatted);

    println!("{}", checksum);
}

fn format_input(input: &str) -> Vec<String> {
    let mut formatted_input = Vec::new();
    for (idx, character) in input.chars().enumerate() {
        if idx % 2 == 0 {
            formatted_input.append(
                &mut std::iter::repeat_n(
                    format!("{}", idx / 2),
                    character.to_digit(10).unwrap().try_into().unwrap(),
                )
                .collect::<Vec<String>>(),
            );
        } else {
            formatted_input.append(
                &mut std::iter::repeat_n(
                    ".".to_string(),
                    character.to_digit(10).unwrap().try_into().unwrap(),
                )
                .collect::<Vec<String>>(),
            );
        }
    }
    formatted_input
}

fn calculate_checksum(mut formatted: Vec<String>) -> usize {
    let mut left = 0;
    let mut right = formatted.len() - 1;

    while left < right {
        if formatted[left] == "." && formatted[right] != "." {
            formatted.swap(left, right);
            left += 1;
            right -= 1;
        } else if formatted[left] != "." {
            left += 1;
        } else {
            right -= 1;
        }
    }

    formatted
        .into_iter()
        .filter_map(|val| val.parse::<usize>().ok())
        .enumerate()
        .map(|(idx, val)| idx * val)
        .sum()
}
