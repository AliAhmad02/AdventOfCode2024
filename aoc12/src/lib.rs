use std::collections::HashMap;

pub fn run(input: &str) -> (usize, usize) {
    let grid = Grid::build(input).unwrap();
    let graph = grid.generate_adjecency_list();
    let regions = find_regions(&graph);

    calculate_total_price(&grid, &graph, &regions)
}

fn calculate_total_price(
    grid: &Grid,
    graph: &HashMap<usize, Vec<usize>>,
    regions: &HashMap<usize, usize>,
) -> (usize, usize) {
    let mut areas: HashMap<usize, usize> = HashMap::new();
    let mut perimeters: HashMap<usize, usize> = HashMap::new();
    let mut sides: HashMap<usize, usize> = HashMap::new();

    for (node, &region) in regions {
        let area = areas.entry(region).or_default();
        *area += 1;

        let num_neighbors = graph.get(node).unwrap().len();
        let perimeter = perimeters.entry(region).or_default();
        *perimeter += 4 - num_neighbors;

        let side = sides.entry(region).or_default();
        *side += grid.num_corners(*node);
    }

    let total_price_p1 = areas
        .iter()
        .map(|(region, area)| area * perimeters.get(region).unwrap())
        .sum();

    let total_price_p2 = areas
        .iter()
        .map(|(region, area)| area * sides.get(region).unwrap())
        .sum();

    (total_price_p1, total_price_p2)
}

fn find_regions(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    let mut counter = 0;
    let mut regions = HashMap::new();

    for &node in graph.keys() {
        if !regions.contains_key(&node) {
            counter += 1;
            visit(graph, node, counter, &mut regions);
        }
    }
    regions
}

fn visit(
    graph: &HashMap<usize, Vec<usize>>,
    node: usize,
    counter: usize,
    regions: &mut HashMap<usize, usize>,
) {
    if regions.contains_key(&node) {
        return;
    }
    regions.insert(node, counter);

    for &next_node in graph.get(&node).unwrap() {
        visit(graph, next_node, counter, regions);
    }
}

#[derive(Clone, Copy)]
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
            let mut neighbors = self.get_neighbors(&position);
            neighbors.retain(|&neigh| self.get_value_from_index(neigh) == character);
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

    fn num_corners(&self, idx: usize) -> usize {
        let position = self.index_to_position(idx);
        let up = Position(position.0 - 1, position.1);
        let down = Position(position.0 + 1, position.1);
        let left = Position(position.0, position.1 - 1);
        let right = Position(position.0, position.1 + 1);

        let value = self.values.chars().nth(idx).unwrap();

        let neighbor_exists = |neigh: &Position| {
            if let Some(idx) = self.position_to_index(neigh) {
                self.values.chars().nth(idx).unwrap() == value
            } else {
                false
            }
        };

        let pairs = [(up, left), (up, right), (down, left), (down, right)];
        let mut corners = 0;

        for (dir1, dir2) in pairs {
            let dir1_exists = neighbor_exists(&dir1);
            let dir2_exists = neighbor_exists(&dir2);

            let diag_exists = neighbor_exists(&Position(dir1.0, dir2.1));

            corners += (!dir1_exists && !dir2_exists) as usize;
            corners += (dir1_exists && dir2_exists && !diag_exists) as usize;
        }
        corners
    }

    fn get_value_from_index(&self, idx: usize) -> char {
        self.values.chars().nth(idx).unwrap()
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
    fn check_adjacency_list() {
        let input = "AA\nBB";
        let grid = Grid::build(input).unwrap();
        let list = HashMap::from([(0, vec![1]), (1, vec![0]), (2, vec![3]), (3, vec![2])]);

        assert_eq!(grid.generate_adjecency_list(), list);
    }

    #[test]
    fn check_find_regions() {
        let input = "\
AAAA
BBCD
BBCC
EEEC";
        let grid = Grid::build(input).unwrap();
        let list = grid.generate_adjecency_list();
        let regions: HashMap<usize, usize> = find_regions(&list);
        assert!([0, 1, 2, 3]
            .iter()
            .map(|node| *regions.get(node).unwrap())
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|w| w[0] == w[1]));

        assert!([4, 5, 8, 9]
            .iter()
            .map(|node| *regions.get(node).unwrap())
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|w| w[0] == w[1]));

        assert!([6, 10, 11, 15]
            .iter()
            .map(|node| *regions.get(node).unwrap())
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|w| w[0] == w[1]));

        assert!([7]
            .iter()
            .map(|node| *regions.get(node).unwrap())
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|w| w[0] == w[1]));

        assert!([12, 13, 14]
            .iter()
            .map(|node| *regions.get(node).unwrap())
            .collect::<Vec<usize>>()
            .windows(2)
            .all(|w| w[0] == w[1]));
    }

    #[test]
        fn check_num_corners() {
            let input = "\
AAAA
BBCD
BBCC
EEEC";
            let grid = Grid::build(input).unwrap();

            assert_eq!(grid.num_corners(4), 1);
            assert_eq!(grid.num_corners(6), 2);
            assert_eq!(grid.num_corners(1), 0);
            assert_eq!(grid.num_corners(10), 2);
            assert_eq!(grid.num_corners(7), 4);
    }

    #[test]
    fn check_calculate_total_price() {
        let input = "\
AAAA
BBCD
BBCC
EEEC";
        let grid = Grid::build(input).unwrap();
        let list = grid.generate_adjecency_list();
        let regions: HashMap<usize, usize> = find_regions(&list);
        let (price1, price2) = calculate_total_price(&grid, &list, &regions);

        assert_eq!(140, price1);
        assert_eq!(80, price2);

    }

}
