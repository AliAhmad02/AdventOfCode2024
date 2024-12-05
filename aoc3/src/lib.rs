use std::error::Error;
use std::fs;

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let instruction = fs::read_to_string(file_path)?;
    let mul = total_mul(&instruction);
    let mul_do = total_mul_do(&instruction);

    println!("Total multiplication: {}", mul);
    println!("Total multiplication (with do/don't): {}", mul_do);

    Ok(())
}

fn total_mul_do(instruction: &str) -> i32 {
    let inst_vec: Vec<i32> = instruction
        .replace("do()", "mul(-1,1)")
        .replace("don't()", "mul(-1,2)")
        .split("mul(")
        .map(|elem| elem.split(")"))
        .flatten()
        .map(|elem| {
            let mut iter = elem.split(",");
            match (iter.next(), iter.next(), iter.next()) {
                (Some(a), Some(b), None) => {
                    a.parse::<i32>().unwrap_or(0) * b.parse::<i32>().unwrap_or(0)
                },
                _ => 0
            }
        })
        .collect();

    let mut do_inst = true;
    let mut total = 0;

    for num in inst_vec {
        if num == -1 {
            do_inst = true;
        } else if num == -2 {
            do_inst = false;
        } else if do_inst {
            total += num;
        }
    }

    total
}

fn total_mul(instruction: &str) -> i32 {
    instruction
        .split("mul(")
        .map(|elem| elem.split(")"))
        .flatten()
        .map(|elem| {
            let mut iter = elem.split(",");
            match (iter.next(), iter.next(), iter.next()) {
                (Some(a), Some(b), None) => {
                    a.parse::<i32>().unwrap_or(0) * b.parse::<i32>().unwrap_or(0)
                },
                _ => 0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_total_mul() {
        let instruction = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(total_mul(instruction), 161);
    }

    #[test]
    fn check_total_mul_do() {
        let instruction = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(total_mul_do(instruction), 48);
    }
}
