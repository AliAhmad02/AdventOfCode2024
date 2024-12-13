use std::collections::{HashMap, HashSet};

pub fn total_score(input: &str) -> Result<(usize, usize), &'static str> {
    let grid = Grid::build(input)?;
    let graph = grid.generate_adjecency_list();

    let mut total_score_p1 = 0;
    let mut total_score_p2 = 0;

    for &node in graph.keys() {
        if grid.get_value_from_index(node) == 0 {
            total_score_p1 += dfs_p1(&grid, &graph, node);
            total_score_p2 += dfs_p2(&grid, &graph, node);

        }
    }
    Ok((total_score_p1, total_score_p2))
}

fn dfs_p1(grid: &Grid, graph: &HashMap<usize, Vec<usize>>, start_node: usize) -> usize {
    let mut visited = HashSet::new();
    let mut score = 0;
    visit(grid, graph, &mut score, start_node, &mut visited);
    score
}

fn dfs_p2(grid: &Grid, graph: &HashMap<usize, Vec<usize>>, start_node: usize) -> usize {
    let mut score = 0;
    backtrack(grid, graph, &mut score, start_node);
    score
}

fn visit(grid: &Grid, graph: &HashMap<usize, Vec<usize>>, score: &mut usize, node: usize, visited: &mut HashSet<usize>) {
    if visited.contains(&node) {
        return;
    }
    if grid.get_value_from_index(node) == 9 {
        *score += 1;
    }
    visited.insert(node);

    for next_node in graph.get(&node).unwrap() {
        visit(grid, graph, score, *next_node, visited);
    }
}

fn backtrack(grid: &Grid, graph: &HashMap<usize, Vec<usize>>, score: &mut usize, node: usize) {
    if graph.get(&node).unwrap().is_empty() {
        if grid.get_value_from_index(node) == 9 {
            *score += 1;
        }
        return;
    }
    for next_node in graph.get(&node).unwrap() {
        backtrack(grid, graph, score, *next_node);
    }
}

struct Position(i32, i32);

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

    fn generate_adjecency_list(&self) -> HashMap<usize, Vec<usize>> {
        let mut list = HashMap::new();

        for (idx, character) in self.values.chars().enumerate() {
            let position = self.index_to_position(idx);
            let value = character.to_digit(10).unwrap();
            let mut neighbors = self.get_neighbors(&position);
            neighbors.retain(|&neigh| {
                if let Some(diff) = self.get_value_from_index(neigh).checked_sub(value) {
                    diff == 1
                } else {
                    false
                }
            });
            list.insert(idx, neighbors);
        }
        list
    }

    fn get_neighbors(&self, position: &Position) -> Vec<usize> {
        let up = Position(position.0 - 1, position.1);
        let down = Position(position.0 + 1, position.1);
        let left = Position(position.0, position.1 - 1);
        let right = Position(position.0, position.1 + 1);

        let neighbors = vec![up, down, left, right];
        neighbors
            .into_iter()
            .filter_map(|pos| self.position_to_index(&pos))
            .collect::<Vec<usize>>()
    }

    fn get_value_from_index(&self, idx: usize) -> u32 {
        self.values.chars().nth(idx).unwrap().to_digit(10).unwrap()
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
        (position.0 < self.n.try_into().unwrap())
            && (position.0 >= 0)
            && (position.1 < self.m.try_into().unwrap())
            && (position.1 >= 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_neighbors() {
        let input = "0123\n1234\n8765\n9876";
        let grid = Grid::build(input).unwrap();
        let first = Position(0, 0);
        let middle = Position(1, 2);
        let last = Position(3, 3);

        assert_eq!(grid.get_neighbors(&first), vec![4, 1]);
        assert_eq!(grid.get_neighbors(&middle), vec![2, 10, 5, 7]);
        assert_eq!(grid.get_neighbors(&last), vec![11, 14]);
    }

    #[test]
    fn check_adjacency_list() {
        let input = "01\n12";
        let grid = Grid::build(input).unwrap();
        let mut list = HashMap::new();
        list.insert(0, vec![2, 1]);
        list.insert(1, vec![3]);
        list.insert(2, vec![3]);
        list.insert(3, vec![]);

        assert_eq!(grid.generate_adjecency_list(), list);
    }

    #[test]
    fn check_dfs_p1() {
        let input = "\
0123
1234
8765
9876";
        let grid = Grid::build(input).unwrap();
        let graph: HashMap<usize, Vec<usize>> = grid.generate_adjecency_list();
        assert_eq!(dfs_p1(&grid, &graph, 0), 1);

    }

    #[test]
    fn check_dfs_p2() {
        let input = "\
2222202
2243212
2252222
2265432
2272242
2287652
2292222";
        let grid = Grid::build(input).unwrap();
        let graph = grid.generate_adjecency_list();
        let start_node = 5;
        assert_eq!(dfs_p2(&grid, &graph, start_node), 3)
    }

    #[test]
    fn check_total_score() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        
        let (score1, score2) = total_score(input).unwrap();

        assert_eq!(score1, 36);
        assert_eq!(score2, 81);
    }

}
