use std::collections::{HashMap, HashSet};
use std::error::Error;

pub struct Grid {
    pub m: i32,
    pub n: i32,
    pub values: String,
}

impl Grid {
    pub fn build(input: &str) -> Result<Grid, Box<dyn Error>> {
        let values = input.replace("\n", "");
        let m: i32 = input.lines().count().try_into()?;
        let mut iter = input.lines().map(|row| row.chars().count());
        let n: i32 = match iter.next() {
            Some(x) => {
                if iter.all(|elem| elem == x) {
                    i32::try_from(x)?
                } else {
                    return Err("Invalid grid! Inconsistent number of rows per column.".into());
                }
            }
            None => 0,
        };

        Ok(Self { m, n, values })
    }
    
    pub fn total_antinodes(&self) -> (usize, usize) {
        let mapping = self.get_char_mapping();
        let mut antinodes: HashSet<i32> = HashSet::new();
        let mut antinodes_all: HashSet<i32> = HashSet::new();

        for (_, positions) in mapping {
            self.add_antinodes_from_antenna(positions, &mut antinodes, &mut antinodes_all);
        }
        (antinodes.len(), antinodes_all.len())
    }

    fn check_position(&self, position: &Position) -> bool {
        (position.0 < self.n) && (position.0 >= 0) && (position.1 < self.m) && (position.1 >= 0)
    }

    fn index_to_position(&self, idx: i32) -> Position {
        Position(idx / self.n, idx % self.n)
    }

    fn position_to_index(&self, position: &Position) -> i32 {
        self.n * position.0 + position.1
    }

    fn get_char_mapping(&self) -> HashMap<char, Vec<i32>> {
        let mut mapping: HashMap<char, Vec<i32>> = HashMap::new();
        for (idx, entry) in self.values.chars().enumerate() {
            if entry != '.' {
                let map_entry = mapping.entry(entry).or_default();
                map_entry.push(idx.try_into().unwrap())
            }
        }

        mapping
    }

    fn add_antinodes_from_antenna(
        &self,
        positions: Vec<i32>,
        antinodes: &mut HashSet<i32>,
        antinodes_all: &mut HashSet<i32>,
    ) {
        for (i, idx1) in positions.iter().enumerate() {
            for idx2 in positions[i + 1..].iter() {
                let (node1, node2) = self.antinodes_from_pair(*idx1, *idx2);
                let nodes_all = self.all_antinodes_from_pair(*idx1, *idx2);

                let mut insert = |node| match node {
                    Some(x) => antinodes.insert(self.position_to_index(&x)),
                    None => false,
                };

                insert(node1);
                insert(node2);

                for entry in nodes_all {
                    antinodes_all.insert(self.position_to_index(&entry));
                }
            }
        }
    }

    fn all_antinodes_from_pair(&self, idx1: i32, idx2: i32) -> Vec<Position> {
        let point1 = self.index_to_position(idx1);
        let point2 = self.index_to_position(idx2);

        let dx = point2.0 - point1.0;
        let dy = point2.1 - point1.1;

        let mut i = 0;

        let mut positions = Vec::new();

        loop {
            let forward_position = Position(point1.0 + i * dx, point1.1 + i * dy);
            if self.check_position(&forward_position) {
                positions.push(forward_position);
                i += 1;
            } else {
                break;
            }
        }

        i = 0;

        loop {
            let backward_position = Position(point1.0 - i * dx, point1.1 - i * dy);
            if self.check_position(&backward_position) {
                positions.push(backward_position);
                i += 1;
            } else {
                break;
            }
        }

        positions
    }

    fn antinodes_from_pair(&self, idx1: i32, idx2: i32) -> (Option<Position>, Option<Position>) {
        let point1 = self.index_to_position(idx1);
        let point2 = self.index_to_position(idx2);

        let dx = point2.0 - point1.0;
        let dy = point2.1 - point1.1;

        let node1 = Position(point1.0 + 2 * dx, point1.1 + 2 * dy);
        let node2 = Position(point1.0 - dx, point1.1 - dy);

        let check_node = |node| {
            if self.check_position(&node) {
                Some(node)
            } else {
                None
            }
        };

        (check_node(node1), check_node(node2))
    }
}

struct Position(i32, i32);
