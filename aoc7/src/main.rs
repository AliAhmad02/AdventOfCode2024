use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let mut total_target = 0;
    let mut total_target_with_concat = 0;
    for line in input.lines() {
        let (target, row) = match line.split_once(":") {
            Some((x, y)) => (
                x.parse::<u64>().unwrap(),
                y.split_whitespace()
                    .map(|elem| elem.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            ),
            _ => panic!("Could not parse row"),
        };
        if check_row(&row, target) {
            total_target += target;
            total_target_with_concat += target;
        } else if check_row_with_concat(&row, target) {
            total_target_with_concat += target;
        }
    }

    println!("Total true calibration results: {}", total_target);
    println!(
        "Total true calibration results (with concatenation): {}",
        total_target_with_concat
    );
}

fn check_row_with_concat(row: &Vec<u64>, target: u64) -> bool {
    let mut sol = vec![row[0]];
    let mut result: HashSet<u64> = HashSet::new();
    backtrack_with_concat(1, &mut sol, &mut result, row);
    result.contains(&target)
}
fn check_row(row: &Vec<u64>, target: u64) -> bool {
    let mut sol = vec![row[0]];
    let mut result: HashSet<u64> = HashSet::new();
    backtrack(1, &mut sol, &mut result, row);
    result.contains(&target)
}

fn backtrack_with_concat(
    i: usize,
    sol: &mut Vec<u64>,
    result: &mut HashSet<u64>,
    input: &Vec<u64>,
) {
    if i == input.len() {
        result.insert(sol[sol.len() - 1]);
        return;
    }

    sol.push(concatenate_integers(sol[sol.len() - 1], input[i]));
    backtrack_with_concat(i + 1, sol, result, input);
    sol.pop().unwrap();

    sol.push(sol[sol.len() - 1] * input[i]);
    backtrack_with_concat(i + 1, sol, result, input);
    sol.pop().unwrap();

    sol.push(sol[sol.len() - 1] + input[i]);
    backtrack_with_concat(i + 1, sol, result, input);
    sol.pop().unwrap();
}

fn concatenate_integers(x: u64, y: u64) -> u64 {
    x * (10_u64).pow(1 + y.ilog10()) + y
}

fn backtrack(i: usize, sol: &mut Vec<u64>, result: &mut HashSet<u64>, input: &Vec<u64>) {
    if i == input.len() {
        result.insert(sol[sol.len() - 1]);
        return;
    }

    sol.push(sol[sol.len() - 1] * input[i]);
    backtrack(i + 1, sol, result, input);
    sol.pop().unwrap();

    sol.push(sol[sol.len() - 1] + input[i]);
    backtrack(i + 1, sol, result, input);
    sol.pop().unwrap();
}
