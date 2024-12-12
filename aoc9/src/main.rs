use std::{collections::HashSet, fs};

fn main() {
    let mut input = fs::read_to_string("data.txt").unwrap();
    let formatted = format_input(&input);
    let mut visited = HashSet::new();
    let checksum = calculate_checksum(formatted.clone());
    let checksum_p2 = calculate_checksum_p2(formatted, &mut input, &mut visited);
    println!("Checksum problem 1: {}", checksum);
    println!("Checksum problem 2: {}", checksum_p2);
}

fn calculate_checksum_p2(
    mut formatted: Vec<String>,
    input: &mut String,
    visited: &mut HashSet<String>,
) -> usize {
    let mut right = formatted.len() - 1;

    while right > 0 {
        if formatted[right] == "." || visited.contains(&formatted[right]) {
            right -= 1;
            continue;
        }

        if let Some((left, size, remainder)) = find_leftmost_space(&formatted, right, input) {
            let space_idx = 2 * formatted[left + 1].parse::<usize>().unwrap() - 1;
            let (slicel, slicer) = formatted.split_at_mut(right - size + 1);

            slicel[left - size - remainder + 1..left - remainder + 1]
                .swap_with_slice(&mut slicer[..size]);

            input.replace_range(space_idx..space_idx + 1, &remainder.to_string());
            visited.insert(formatted[right].clone());
            right -= size;
        } else {
            right -= 1;
        }
    }

    formatted
        .into_iter()
        .enumerate()
        .filter_map(|(idx, val)| val.parse::<usize>().ok().map(|parsed| parsed * idx))
        .sum()
}

fn find_leftmost_space(
    formatted: &[String],
    right: usize,
    input: &str,
) -> Option<(usize, usize, usize)> {
    let mut left = 0;
    let block_idx = 2 * formatted[right].parse::<usize>().unwrap();
    let block_size = input.chars().nth(block_idx).unwrap().to_digit(10).unwrap();
    let mut block = None;

    while left < right {
        if formatted[left] == "." && formatted[left + 1] != "." {
            let space_idx = 2 * formatted[left + 1].parse::<usize>().unwrap() - 1;
            let space = input.chars().nth(space_idx).unwrap().to_digit(10).unwrap();

            match space.checked_sub(block_size) {
                Some(remainder) => {
                    block = Some((left, block_size as usize, remainder as usize));
                    break;
                }
                None => {
                    left += 1;
                }
            }
        } else {
            left += 1;
        }
    }
    block
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
        .enumerate()
        .filter_map(|(idx, val)| val.parse::<usize>().ok().map(|parsed| parsed * idx))
        .sum()
}
