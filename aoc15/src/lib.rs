use std::collections::HashSet;
use std::fmt;
use std::ops::Add;

pub fn run(input: &str) {
    let (mut grid, mut grid_expanded, moves) = parse_string(input);

    moves.chars().for_each(|direction| {
        make_move(&mut grid, direction);
        make_move_expanded(&mut grid_expanded, direction);
    });

    let gps_coords_sum = sum_of_gps_coords(&grid);
    let gps_coords_expanded_sum = sum_of_gps_coords_expanded(&grid_expanded);

    println!("Sum of GPS coordinates (problem 1): {}", gps_coords_sum);
    println!(
        "Sum of GPS coordinates (problem 2): {}",
        gps_coords_expanded_sum
    );
}

fn make_move_expanded(grid: &mut Grid, direction: char) {
    let robot_idx = grid.values.find("@").unwrap();
    let robot_pos = grid.index_to_position(robot_idx);

    let left = Position(0, -1);
    let right = Position(0, 1);
    let up = Position(-1, 0);
    let down = Position(1, 0);

    let move_direction = match direction {
        '^' => &up,
        'v' => &down,
        '<' => &left,
        '>' => &right,
        _ => panic!("Invalid move command."),
    };

    let next_pos = &robot_pos + move_direction;
    let next_idx = grid.position_to_index(&next_pos).unwrap();
    let next_char = grid.values.chars().nth(next_idx).unwrap();

    if next_char == '#' {
        return;
    }

    let mut paren_indices = Vec::new();

    if next_char == '[' || next_char == ']' {
        paren_indices.push(next_idx)
    }

    let mut paren_seen: HashSet<usize> = HashSet::new();

    while let Some(paren_idx) = paren_indices.pop() {
        let paren_pos = grid.index_to_position(paren_idx);
        let paren_match = match grid.values.chars().nth(paren_idx).unwrap() {
            '[' => grid.position_to_index(&(&paren_pos + &right)).unwrap(),
            ']' => grid.position_to_index(&(&paren_pos + &left)).unwrap(),
            _ => panic!("Parenthesis error!"),
        };

        if !paren_seen.contains(&paren_match) {
            paren_indices.push(paren_match);
            paren_seen.insert(paren_match);
        }

        let paren_next_pos = &paren_pos + move_direction;
        let paren_next_idx = grid.position_to_index(&paren_next_pos).unwrap();
        let paren_next_char = grid.values.chars().nth(paren_next_idx).unwrap();

        if paren_next_char == '[' || paren_next_char == ']' {
            paren_indices.push(paren_next_idx);
            paren_seen.insert(paren_next_idx);
        }

        if paren_next_char == '#' {
            return;
        }
    }

    let mut grid_as_vec: Vec<_> = grid.values.chars().collect();

    let mut paren_indices: Vec<_> = paren_seen.into_iter().collect();
    if !paren_indices.is_empty() {
        paren_indices.sort_by_key(|&index| {
            let pos = grid.index_to_position(index);
            (robot_pos.0 - pos.0).pow(2) + (robot_pos.1 - pos.1).pow(2)
        });
        for idx in paren_indices.into_iter().rev() {
            let paren_pos = grid.index_to_position(idx);
            let paren_pos_next = &paren_pos + move_direction;
            let paren_idx_next = grid.position_to_index(&paren_pos_next).unwrap();

            grid_as_vec.swap(idx, paren_idx_next);
        }
    }

    grid_as_vec.swap(robot_idx, next_idx);

    grid.values = grid_as_vec.into_iter().collect();
}

fn sum_of_gps_coords(grid: &Grid) -> i32 {
    grid.values
        .chars()
        .enumerate()
        .filter_map(|(idx, character)| {
            if character == 'O' {
                Some(calculate_gps_coordinate(&grid.index_to_position(idx)))
            } else {
                None
            }
        })
        .sum()
}

fn sum_of_gps_coords_expanded(grid: &Grid) -> i32 {
    grid.values
        .chars()
        .enumerate()
        .filter_map(|(idx, character)| {
            if character == '[' {
                Some(calculate_gps_coordinate(&grid.index_to_position(idx)))
            } else {
                None
            }
        })
        .sum()
}

fn calculate_gps_coordinate(box_pos: &Position) -> i32 {
    100 * box_pos.0 + box_pos.1
}

fn make_move(grid: &mut Grid, direction: char) {
    let robot_idx = grid.values.find("@").unwrap();
    let robot_pos = grid.index_to_position(robot_idx);

    let left = Position(0, -1);
    let right = Position(0, 1);
    let up = Position(-1, 0);
    let down = Position(1, 0);

    let move_direction = match direction {
        '^' => &up,
        'v' => &down,
        '<' => &left,
        '>' => &right,
        _ => panic!("Invalid move command."),
    };

    let mut next_pos = &robot_pos + move_direction;

    let mut next_char = grid
        .values
        .chars()
        .nth(grid.position_to_index(&next_pos).unwrap())
        .unwrap();

    let mut swap_indices = vec![
        grid.position_to_index(&robot_pos).unwrap(),
        grid.position_to_index(&next_pos).unwrap(),
    ];

    while next_char == 'O' {
        next_pos = &next_pos + move_direction;

        swap_indices.push(grid.position_to_index(&next_pos).unwrap());

        next_char = grid
            .values
            .chars()
            .nth(grid.position_to_index(&next_pos).unwrap())
            .unwrap();
    }
    if next_char == '.' {
        swap_positions(grid, &swap_indices);
    }
}

fn swap_positions(grid: &mut Grid, swap_indices: &[usize]) {
    let mut values_vec = grid.values.chars().collect::<Vec<_>>();
    for window in swap_indices.windows(2).rev() {
        values_vec.swap(window[0], window[1]);
    }
    grid.values = values_vec.into_iter().collect();
}

fn parse_string(input: &str) -> (Grid, Grid, String) {
    let (grid_str, moves) = input.split_once("\n\n").unwrap();
    (
        Grid::build(grid_str).unwrap(),
        Grid::build_expanded(grid_str).unwrap(),
        moves.replace("\n", ""),
    )
}

struct Position(i32, i32);

impl Add for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

struct Grid {
    pub m: usize,
    pub n: usize,
    pub values: String,
}

impl Grid {
    fn build(input: &str) -> Result<Grid, &'static str> {
        let values = input.replace("\n", "");
        let m = input.lines().count();
        let mut iter = input.lines().map(|row| row.chars().count());
        let n = match iter.next() {
            Some(x) => {
                if iter.all(|elem| elem == x) {
                    x
                } else {
                    return Err("Invalid input! Failed to construct grid.");
                }
            }
            None => 0,
        };

        Ok(Self { m, n, values })
    }

    fn build_expanded(input: &str) -> Result<Grid, &'static str> {
        let values = input
            .replace("\n", "")
            .replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.");

        let m = input.lines().count();
        let mut iter = input.lines().map(|row| row.chars().count());
        let n = match iter.next() {
            Some(x) => {
                if iter.all(|elem| elem == x) {
                    x * 2
                } else {
                    return Err("Invalid input! Failed to construct grid.");
                }
            }
            None => 0,
        };

        Ok(Self { m, n, values })
    }

    fn index_to_position(&self, idx: usize) -> Position {
        Position(
            (idx / self.n).try_into().unwrap(),
            (idx % self.n).try_into().unwrap(),
        )
    }

    fn position_to_index(&self, position: &Position) -> Option<usize> {
        match self.check_position(position) {
            true => Some(
                self.n * usize::try_from(position.0).unwrap()
                    + usize::try_from(position.1).unwrap(),
            ),
            false => None,
        }
    }

    fn check_position(&self, position: &Position) -> bool {
        (position.0 < self.m.try_into().unwrap())
            && (position.0 >= 0)
            && (position.1 < self.n.try_into().unwrap())
            && (position.1 >= 0)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self
            .values
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % self.n == 0 {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>();

        write!(f, "{}", formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_make_move() {
        let grid_input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let moves_input = "<^^>>>vv<v>>v<<";

        let mut grid = Grid::build(grid_input).unwrap();

        for direction in moves_input.chars() {
            make_move(&mut grid, direction);
        }
        let final_configuration =
            "#########....OO###.....##.....O##.#O@..##...O..##...O..#########";

        assert_eq!(final_configuration, grid.values);
    }

    #[test]
    fn check_calculate_gps_coordinate() {
        let box_pos = Position(1, 4);
        assert_eq!(calculate_gps_coordinate(&box_pos), 104);
    }

    #[test]
    fn check_sum_of_gps_coords() {
        let input = "\
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";
        let grid = Grid::build(input).unwrap();
        assert_eq!(sum_of_gps_coords(&grid), 10092);
    }

    #[test]
    fn check_build_expanded() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";
        let result_expanded = "\
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";
        let grid_expanded = Grid::build_expanded(input).unwrap();
        assert_eq!(grid_expanded.values, result_expanded.replace("\n", ""));
        assert_eq!(grid_expanded.n, 20);
        assert_eq!(grid_expanded.m, 10);
    }

    #[test]
    fn check_make_move_expanded() {
        let grid_input = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######";
        let moves_input = "<vv<<^^<<^^";

        let mut grid = Grid::build_expanded(grid_input).unwrap();

        for direction in moves_input.chars() {
            make_move_expanded(&mut grid, direction);
        }
        let final_configuration =
            "################...[].##..####...@.[]...####....[]....####..........####..........################";

        assert_eq!(final_configuration, grid.values);
    }

    #[test]
    fn check_sum_of_gps_coords_expanded() {
        let input = "\
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";
        let grid = Grid::build(input).unwrap();
        assert_eq!(sum_of_gps_coords_expanded(&grid), 9021);
    }
}
