pub fn calculate_updated_price(input: &str) -> i64 {
    parse_string(input)
        .filter_map(|args| {
            calculate_button_presses(
                args[0],
                args[1],
                args[2],
                args[3],
                args[4] + 10000000000000,
                args[5] + 10000000000000,
            )
        })
        .map(|(n_a, n_b)| 3 * n_a + n_b)
        .sum()
}

pub fn calculate_total_price(input: &str) -> i64 {
    parse_string(input)
        .filter_map(|args| {
            calculate_button_presses(args[0], args[1], args[2], args[3], args[4], args[5])
        })
        .map(|(n_a, n_b)| 3 * n_a + n_b)
        .sum()
}

fn parse_string(input: &str) -> impl Iterator<Item = [i64; 6]> {
    let string = input
        .chars()
        .filter(|character| character.is_ascii_digit() || character == &'\n' || character == &',')
        .collect::<String>();

    let collected: Vec<[i64; 6]> = string
        .split("\n\n")
        .filter_map(|block| {
            let numbers: Vec<i64> = block
                .lines()
                .flat_map(|line| {
                    line.split(',')
                        .filter_map(|num| num.trim().parse::<i64>().ok())
                })
                .collect();

            if numbers.len() == 6 {
                Some([
                    numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5],
                ])
            } else {
                None
            }
        })
        .collect();

    collected.into_iter()
}

fn calculate_button_presses(
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    x: i64,
    y: i64,
) -> Option<(i64, i64)> {
    let denominator = a_x * b_y - b_x * a_y;
    let numerator_a = x * b_y - b_x * y;
    let numerator_b = y * a_x - a_y * x;

    if (numerator_a % denominator) == 0 && (numerator_b % denominator == 0) {
        Some((numerator_a / denominator, numerator_b / denominator))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_button_presses() {
        let presses1 = calculate_button_presses(94, 34, 22, 67, 8400, 5400);
        let presses2 = calculate_button_presses(67, 21, 12748, 12176, 26, 66);
        let presses3 = calculate_button_presses(17, 86, 84, 37, 7870, 6450);
        let presses4 = calculate_button_presses(69, 23, 27, 71, 18641, 10279);

        assert_eq!(presses1.unwrap().0, 80);
        assert_eq!(presses1.unwrap().1, 40);

        assert!(presses2.is_none());

        assert_eq!(presses3.unwrap().0, 38);
        assert_eq!(presses3.unwrap().1, 86);

        assert!(presses4.is_none());
    }

    #[test]
    fn check_parse_string() {
        let string = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        assert!(
            vec![[94, 34, 22, 67, 8400, 5400], [26, 66, 67, 21, 12748, 12176]]
                .into_iter()
                .eq(parse_string(string))
        );
    }

    #[test]
    fn check_calculate_total_price() {
        let string = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(calculate_total_price(string), 480);
    }
}
