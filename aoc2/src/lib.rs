use std::error::Error;
use std::fs;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let (nsafe, nsafe_loose) = safe_counter(&contents)?;

    println!("Number of safe reports: {}", nsafe);
    println!("Number of safe reports (loose): {}", nsafe_loose);

    Ok(())
}

fn safe_counter(contents: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let mut nsafe: u32 = 0;
    let mut nsafe_loose: u32 = 0;
    for line in contents.lines() {
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|elem| elem.trim().parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;
        if check_row_safety(&row) {
            nsafe += 1;
            nsafe_loose += 1;
        } else if check_row_safety_allow_one(&row) {
            nsafe_loose += 1;
        }
    }

    Ok((nsafe, nsafe_loose))
}

fn check_row_safety_allow_one(row: &Vec<u32>) -> bool {
    for (idx, _) in row.iter().enumerate() {
        let mut row_copy = row.clone();
        row_copy.remove(idx);
        if check_row_safety(&row_copy) {
            return true;
        }
    }

    false
}

fn check_row_safety(row: &Vec<u32>) -> bool {
    let sorted = row.is_sorted_by(|a, b| (a < b) && ((b - a) <= 3));
    let sorted_reverse = row.is_sorted_by(|a, b| (a > b) && ((a - b) <= 3));

    if !sorted && !sorted_reverse {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checker_ascending() {
        assert!(check_row_safety(&vec![1, 2, 3]));
    }

    #[test]
    fn checker_descending() {
        assert!(check_row_safety(&vec![3, 2, 1]));
    }

    #[test]
    fn checker_unsafe_order() {
        assert!(!check_row_safety(&vec![1, 3, 2]));
    }

    #[test]
    fn checker_unsafe_value() {
        assert!(!check_row_safety(&vec![1, 2, 6]));
    }

    #[test]
    fn checker_safe_counter() {
        let contents = "1   2   3\n4    5   10\n12 11 10";
        assert_eq!(2, safe_counter(contents).unwrap().0);
    }

    #[test]
    fn checker_allow_one() {
        assert!(check_row_safety_allow_one(&vec![1, 3, 2, 4, 5]));
    }
}
