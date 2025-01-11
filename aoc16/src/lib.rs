use std::fmt;
use std::ops::{Add, Mul, Sub};

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn run(input: &str) {
    let grid = Grid::build(input).unwrap();
    let start_node = grid.values.find("S").unwrap();
    let end_node = grid.values.find("E").unwrap();
    let start_direction = Position(0, 1);

    let dijkstra_result = dijkstra(&grid, start_node, &start_direction);
    let min_dist = *dijkstra_result.distances.get(&end_node).unwrap();
    let mut min_path = dijkstra_result.get_path_to_node(end_node);
    let n_best_seats = num_best_seats(&grid, end_node, dijkstra_result, &mut min_path);

    println!("The shortest path has length: (Problem 1): {}", min_dist);

    println!("The number of best seats: (Problem 2): {}", n_best_seats);
}

fn num_best_seats(
    grid: &Grid,
    end_node: usize,
    dijkstra_res_start: DijkstraResult,
    best_seats: &mut HashSet<usize>,
) -> usize {
    let min_dist = *dijkstra_res_start.distances.get(&end_node).unwrap();

    for (&idx, &dist_from_start) in dijkstra_res_start.distances.iter() {
        if dist_from_start <= min_dist {
            let final_dir_start = dijkstra_res_start
                .final_directions
                .get(&idx)
                .unwrap_or(&Position(0, 0));

            let dijkstra_res_end = dijkstra(grid, idx, final_dir_start);
            let dist_to_end = *dijkstra_res_end
                .distances
                .get(&end_node)
                .unwrap_or(&u32::MAX);

            if (dist_to_end <= min_dist) && (dist_from_start + dist_to_end == min_dist) {
                best_seats.extend(dijkstra_res_start.get_path_to_node(idx));
                best_seats.extend(dijkstra_res_end.get_path_to_node(end_node));
            }
        }
    }
    best_seats.len()
}

fn dijkstra(grid: &Grid, start_node: usize, start_direction: &Position) -> DijkstraResult {
    let mut directions = HashMap::from([(start_node, *start_direction)]);
    let mut distances = HashMap::from([(start_node, 0)]);
    let mut final_directions = directions.clone();
    let mut previous = HashMap::from([(start_node, None)]);

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0, start_node)));
    let mut visited = HashSet::new();

    while let Some(Reverse((weight, node))) = min_heap.pop() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        final_directions
            .entry(node)
            .or_insert_with(|| *directions.get(&node).unwrap());

        let node_pos = grid.index_to_position(node);
        let direction = directions.get(&node).unwrap();

        for neigh in grid.get_neighbors(node, direction) {
            if let (Some(node_neigh), neigh_weight) = neigh {
                let neigh_pos = grid.index_to_position(node_neigh);
                directions.insert(node_neigh, &neigh_pos - &node_pos);

                let new_distance = weight + neigh_weight;
                let old_distance = distances.entry(node_neigh).or_insert(u32::MAX);

                if &new_distance < old_distance {
                    *old_distance = new_distance;
                    min_heap.push(Reverse((weight + neigh_weight, node_neigh)));
                    previous.insert(node_neigh, Some(node));
                }
            }
        }
    }

    DijkstraResult {
        distances,
        final_directions,
        previous,
    }
}

struct DijkstraResult {
    distances: HashMap<usize, u32>,
    final_directions: HashMap<usize, Position>,
    previous: HashMap<usize, Option<usize>>,
}

impl DijkstraResult {
    fn get_path_to_node(&self, end_node: usize) -> HashSet<usize> {
        let mut node = end_node;
        let mut path = HashSet::from([node]);

        while let Some(prev) = self.previous.get(&node).unwrap_or(&None) {
            path.insert(*prev);
            node = self.previous.get(&node).unwrap().unwrap();
        }

        path
    }
}

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

impl Add for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<i32> for &Position {
    type Output = Position;

    fn mul(self, other: i32) -> Position {
        Position(self.0 * other, self.1 * other)
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

    fn get_neighbors(&self, index: usize, dir: &Position) -> Vec<(Option<usize>, u32)> {
        let pos = self.index_to_position(index);
        // clockwise and counter clockwise directions
        let cw = &(&Position(0, -1) * dir.0) + &(&Position(1, 0) * dir.1);
        let ccw = &(&Position(0, 1) * dir.0) + &(&Position(-1, 0) * dir.1);

        let neighbors = [(&pos + dir, 1), (&pos + &cw, 1001), (&pos + &ccw, 1001)];
        neighbors
            .into_iter()
            .map(|(pos, weight)| {
                let neigh_idx = self
                    .position_to_index(&pos)
                    .filter(|&idx| self.values.chars().nth(idx).unwrap() != '#');
                (neigh_idx, weight)
            })
            .collect()
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
    fn check_dijkstra() {
        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let grid = Grid::build(input).unwrap();
        let start_node = grid.values.find("S").unwrap();
        let end_node = grid.values.find("E").unwrap();
        let start_direction = Position(0, 1);
        let dijkstra_result = dijkstra(&grid, start_node, &start_direction);
        assert_eq!(*dijkstra_result.distances.get(&end_node).unwrap(), 11048)
    }

    #[test]
    fn check_num_best_seats() {
        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let grid = Grid::build(input).unwrap();
        let start_node = grid.values.find("S").unwrap();
        let end_node = grid.values.find("E").unwrap();
        let start_direction = Position(0, 1);
        let dijkstra_result = dijkstra(&grid, start_node, &start_direction);

        let best_seats = num_best_seats(&grid, end_node, dijkstra_result, &mut HashSet::new());
        assert_eq!(best_seats, 64)
    }
}
