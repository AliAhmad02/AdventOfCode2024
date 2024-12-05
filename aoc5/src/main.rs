use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let (rules, pages) = match input.split_once("\n\n") {
        Some((x, y)) => (x.to_string(), y.to_string()),
        _ => ("".to_string(), "".to_string()),
    };

    let rules = rules
        .replace("|", ",")
        .split("\n")
        .map(|elem| match elem.split_once(",") {
            Some((x, y)) => (x.to_string(), y.to_string()),
            _ => ("".to_string(), "".to_string()),
        })
        .collect::<HashSet<(String, String)>>();
    let pages = pages.lines().map(str::to_string).collect::<Vec<String>>();

    let mut total_mid: u32 = 0;
    let mut total_mid_wrong: u32 = 0;

    for page in pages {
        let nums: Vec<String> = page.split(",").map(str::to_string).collect();

        if check_valid(&nums, &rules) {
            total_mid += nums[nums.len() / 2].parse::<u32>().unwrap();
        } else {
            total_mid_wrong += fix_page(&nums, &rules)[nums.len() / 2]
                .parse::<u32>()
                .unwrap();
        }
    }
    println!("Sum of middle page numbers: {}", total_mid);
    println!(
        "Sum of middle page numbers (wrong updates): {}",
        total_mid_wrong
    );
}

fn fix_page(nums: &Vec<String>, lookup: &HashSet<(String, String)>) -> Vec<String> {
    let mut fixed_nums: Vec<String> = vec!["".to_string(); nums.len()];

    for num in nums {
        let n_after = nums
            .iter()
            .filter(|elem| lookup.contains(&(num.to_string(), elem.to_string())) && elem != &num)
            .count();
        let fixed_index = nums.len() - 1 - n_after;
        fixed_nums[fixed_index] = num.to_string();
    }

    fixed_nums
}

fn check_valid(nums: &Vec<String>, lookup: &HashSet<(String, String)>) -> bool {
    nums.iter()
        .enumerate()
        .all(|(i, _)| check_num(nums, lookup, i))
}

fn check_num(nums: &Vec<String>, lookup: &HashSet<(String, String)>, i: usize) -> bool {
    let num = &nums[i];
    let before = &nums[..i];
    let after = &nums[i + 1..];

    let after_zip = vec![num; after.len()].into_iter().zip(after.into_iter());
    let before_zip = before.into_iter().zip(vec![num; before.len()].into_iter());

    let after_b = after_zip
        .filter(|elem| lookup.contains(&(elem.0.to_string(), elem.1.to_string())))
        .count()
        == after.len();
    let before_b = before_zip
        .filter(|elem| lookup.contains(&(elem.0.to_string(), elem.1.to_string())))
        .count()
        == before.len();

    after_b && before_b
}
