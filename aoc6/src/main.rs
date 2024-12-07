use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();

    let lx: i32 = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();
    let ly: i32 = input.lines().count().try_into().unwrap();
    let mut input = input.replace("\n", "");

    let start_idx: i32 = input
        .chars()
        .position(|c| c == '^')
        .unwrap()
        .try_into()
        .unwrap();
    let mut position = index_to_position(start_idx, lx);
    let mut direction = Direction::Up;

    let mut positions: HashSet<i32> = HashSet::new();
    positions.insert(position_to_index(&position, lx));

    while inside_bounds(&position, &direction, lx, ly) {
        (direction, position) = take_step(&input, position, direction, lx);
        positions.insert(position_to_index(&position, lx));
    }

    println!("Number of distinct positions: {}", positions.len());

    let mut obstructions: u32 = 0;

    positions.remove(&start_idx);

    for idx in positions {
        input.replace_range((idx as usize)..(idx as usize) + 1, "#");
        obstructions += check_loop(
            &input,
            index_to_position(start_idx, lx),
            Direction::Up,
            lx,
            ly,
        ) as u32;
        input.replace_range((idx as usize)..(idx as usize) + 1, ".");
    }

    println!("Total number of obstructions: {}", obstructions);
}

fn check_loop(
    input: &str,
    mut position: Position,
    mut direction: Direction,
    lx: i32,
    ly: i32,
) -> bool {
    let mut state: HashSet<(i32, Direction)> = HashSet::new();
    let mut has_loop = false;
    loop {
        if !inside_bounds(&position, &direction, lx, ly) {
            break;
        }
        (direction, position) = take_step(input, position, direction.clone(), lx);
        if state.contains(&(position_to_index(&position, lx), direction.clone())) {
            has_loop = true;
            break;
        }
        state.insert((position_to_index(&position, lx), direction.clone()));
    }

    has_loop
}

fn inside_bounds(position: &Position, direction: &Direction, lx: i32, ly: i32) -> bool {
    let new_position = step_in_direction(position, direction);
    (new_position.0 < lx) && (new_position.0 >= 0) && (new_position.1 < ly) && (new_position.1 >= 0)
}

fn take_step(
    input: &str,
    position: Position,
    mut direction: Direction,
    lx: i32,
) -> (Direction, Position) {
    let mut new_position = step_in_direction(&position, &direction);

    let new_idx = position_to_index(&new_position, lx);
    let next_char = input.chars().nth(new_idx.try_into().unwrap()).unwrap();

    if next_char == '#' {
        direction = change_direction(direction);
        new_position = position;
    }
    (direction, new_position)
}

fn step_in_direction(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => Position(position.0 - 1, position.1),
        Direction::Down => Position(position.0 + 1, position.1),
        Direction::Right => Position(position.0, position.1 + 1),
        Direction::Left => Position(position.0, position.1 - 1),
    }
}

fn change_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

struct Position(i32, i32);

fn index_to_position(idx: i32, lx: i32) -> Position {
    Position(idx / lx, idx % lx)
}

fn position_to_index(position: &Position, lx: i32) -> i32 {
    lx * position.0 + position.1
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
