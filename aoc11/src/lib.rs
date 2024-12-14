use std::collections::HashMap;

pub fn num_stones_after_blinks(input: &str, n_blinks: u32) -> usize {
    let mut input_map: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|elem| (elem.parse::<usize>().unwrap(), 1))
        .collect();

    for _ in 0..n_blinks {
        input_map = perform_blink(&input_map);
    }

    input_map.values().sum()
}

fn perform_blink(map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_map = HashMap::new();
    for (&stone, count) in map {
        for new_stone in transform_stone(stone) {
            let new_stone_entry = new_map.entry(new_stone).or_insert(0);
            *new_stone_entry += count;
        }
    }
    new_map
}

fn transform_stone(stone: usize) -> Vec<usize> {
    let mut new_stones = Vec::new();

    if stone == 0 {
        new_stones.push(1);
    } else if count_digits(stone) % 2 == 0 {
        let (num1, num2) = split_integer(stone);
        new_stones.push(num1);
        new_stones.push(num2);
    } else {
        new_stones.push(stone * 2024);
    }

    new_stones
}

fn split_integer(integer: usize) -> (usize, usize) {
    let mut num1 = 0;
    let mut num2 = 0;

    let mut temp_vec = Vec::new();

    let n_digits = count_digits(integer);

    for i in 0..n_digits {
        temp_vec.push((integer / 10_usize.pow(i.try_into().unwrap())) % 10);
        if i == (n_digits / 2 - 1) {
            num1 += concatenate_digits(&temp_vec);
            temp_vec.clear();
        }
    }
    num2 += concatenate_digits(&temp_vec);

    (num2, num1)
}

fn concatenate_digits(rev_digits: &[usize]) -> usize {
    rev_digits.iter().rev().fold(0, |acc, elem| acc * 10 + elem)
}

fn count_digits(integer: usize) -> usize {
    if integer == 0 {
        1
    } else {
        (integer.ilog10() + 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_count_digits() {
        let int0 = 0;
        let int1 = 123;
        let int2 = 9999;
        let int3 = 2910805982;

        assert_eq!(count_digits(int0), 1);
        assert_eq!(count_digits(int1), 3);
        assert_eq!(count_digits(int2), 4);
        assert_eq!(count_digits(int3), 10);
    }

    #[test]
    fn check_concatenate_digits() {
        let digits1 = vec![4, 3, 2, 1];
        let digits2 = vec![4, 3, 2, 0];
        assert_eq!(concatenate_digits(&digits1), 1234);
        assert_eq!(concatenate_digits(&digits2), 234);
    }

    #[test]
    fn check_split_integer() {
        let integer1 = 1000;
        let integer2 = 12345;
        assert_eq!((10, 0), split_integer(integer1));
        assert_eq!((123, 45), split_integer(integer2));
    }

    #[test]
    fn check_perform_blink() {
        let input_map = HashMap::from([(125, 1), (17, 1)]);
        assert_eq!(
            perform_blink(&input_map),
            HashMap::from([(253000, 1), (1, 1), (7, 1)])
        );
    }

    #[test]
    fn check_num_stones_after_blinks() {
        let input = "125 17";
        let n_blinks = 25;
        assert_eq!(num_stones_after_blinks(input, n_blinks), 55312);
    }
}
