pub fn problem2(input: &str) -> (i64, f64) {
    let mut robots = parse_string(input);

    let tw: i64 = 101;
    let th: i64 = 103;

    let mut max_entropy = 0.0;
    let mut best_iteration = 0;
    let mut christmas_tree = "".to_string();
    for i in 1..tw * th {
        move_robots(&mut robots, 1, tw, th);
        let robots_as_string = robots_to_string(&robots, tw, th);
        let entropy = string_entropy(&robots_as_string);
        if entropy > max_entropy {
            max_entropy = entropy;
            best_iteration = i;
            christmas_tree = robots_as_string;
        }
    }

    println!("CHRISTMAS TREE!\n{}", christmas_tree);

    (best_iteration, max_entropy)
}

fn string_entropy(string: &str) -> f64 {
    let string_no_spaces = string.replace("\n", "");
    let dots = string_no_spaces
        .chars()
        .filter(|&character| character == '.')
        .count() as f64;
    let hashtags = string_no_spaces
        .chars()
        .filter(|&character| character == '#')
        .count() as f64;

    let p_dots = dots / string_no_spaces.len() as f64;
    let p_hashtags = hashtags / string_no_spaces.len() as f64;

    -(p_dots * p_dots.log2() + p_hashtags * p_hashtags.log2())
}

fn robots_to_string(robots: &Vec<Robot>, tw: i64, th: i64) -> String {
    let mut string_vec = vec!["."; (tw * th).try_into().unwrap()];
    for robot in robots {
        string_vec[(robot.px + robot.py * tw) as usize] = "#";
    }

    string_vec
        .chunks(tw as usize)
        .map(|chunk| {
            chunk
                .iter()
                .map(|elem| elem.to_string())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn problem1(input: &str) -> i64 {
    let mut robots = parse_string(input);
    let seconds = 100;
    let tw = 101;
    let th = 103;

    move_robots(&mut robots, seconds, tw, th);

    calculate_safety_factor(&robots, tw, th)
}

fn calculate_safety_factor(robots: &[Robot], tw: i64, th: i64) -> i64 {
    let mut quadrants = [0, 0, 0, 0];
    for robot in robots {
        if (tw % 2 == 1 && robot.px == tw / 2) || (th % 2 == 1 && robot.py == th / 2) {
            continue;
        }

        let quadrant = (robot.py > th / 2) as usize * 2 + (robot.px > tw / 2) as usize;
        quadrants[quadrant] += 1;
    }

    quadrants.into_iter().product()
}

fn move_robots(robots: &mut [Robot], seconds: i64, tw: i64, th: i64) {
    for _ in 0..seconds {
        robots
            .iter_mut()
            .for_each(|robot| robot.update_position(tw, th));
    }
}

#[derive(Clone)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn update_position(&mut self, tw: i64, th: i64) {
        self.px = (self.px + self.vx).rem_euclid(tw);
        self.py = (self.py + self.vy).rem_euclid(th);
    }
}

fn parse_string(input: &str) -> Vec<Robot> {
    let cleaned = input.replace(" v=", ",").replace("p=", "");
    cleaned
        .lines()
        .filter_map(|line| {
            let numbers = line
                .split(",")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if numbers.len() == 4 {
                Some(Robot {
                    px: numbers[0],
                    py: numbers[1],
                    vx: numbers[2],
                    vy: numbers[3],
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_move_robots() {
        let mut robots = vec![Robot {
            px: 2,
            py: 4,
            vx: 2,
            vy: -3,
        }];
        move_robots(&mut robots, 5, 11, 7);
        assert_eq!(robots[0].px, 1);
        assert_eq!(robots[0].py, 3);
    }

    #[test]
    fn check_calculate_safety_factor() {
        let robots = vec![
            Robot {
                px: 6,
                py: 0,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 6,
                py: 0,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 9,
                py: 0,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 0,
                py: 2,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 1,
                py: 3,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 2,
                py: 3,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 5,
                py: 4,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 3,
                py: 5,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 4,
                py: 5,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 4,
                py: 5,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 1,
                py: 6,
                vx: 0,
                vy: 0,
            },
            Robot {
                px: 6,
                py: 6,
                vx: 0,
                vy: 0,
            },
        ];
        assert_eq!(calculate_safety_factor(&robots, 11, 7), 12)
    }

    #[test]
    fn check_string_entropy() {
        let string = "#.##.#.#";
        let entropy = string_entropy(string);

        assert_eq!((entropy * 1000.0).round() / 1000.0, 0.954);
    }
}
