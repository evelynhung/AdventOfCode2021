use std::{
    collections::{BinaryHeap, HashMap},
    io::{BufRead, BufReader},
};

use advent_of_code::read_file_to_string;

const DIRECTION: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

struct Map {
    grid: Vec<Vec<u8>>,
    extended: bool,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(grid: Vec<Vec<u8>>, extended: bool) -> Map {
        let (rows, cols) = (grid.len(), grid[0].len());
        Map { grid, extended, rows, cols }
    }
    fn get_val(&self, i: usize, j: usize) -> i32 {
        match self.extended {
            false => (self.grid[i][j] - b'0') as i32,
            true => {
                let val = (self.grid[i % self.rows][j % self.cols] - b'0') as i32;
                let distance = (i / self.rows + j / self.cols) as i32;
                (val + distance - 1) % 9 + 1
            },
        }
    }

    fn on_map(&self, i: isize, j: isize) -> bool {
        let (rows, cols) = match self.extended {
            false => (self.rows as isize, self.cols as isize),
            true => (5 * self.rows as isize, 5 * self.cols as isize)
        };
        i >= 0 && j >= 0 && i < rows && j < cols
    }

    fn goal(&self) -> (usize, usize) {
        match self.extended {
            false => (self.rows - 1, self.cols - 1),
            true => (5 * self.rows - 1, 5 * self.cols - 1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    BufReader::new(input.as_bytes())
        .lines()
        .map(|line| line.unwrap().trim().as_bytes().to_vec())
        .collect()
}

fn find_shortest_path(map: &Map) -> i32 {
    let mut seen: HashMap<(usize, usize), i32> = HashMap::new();

    let goal = map.goal();

    let mut max_heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
    max_heap.push((0, 0, 0));
    seen.entry((0, 0)).or_default();

    while !max_heap.is_empty() {
        let (risk, x, y) = max_heap.pop().unwrap();

        if (x, y) == goal {
            return -risk;
        }

        for (dx, dy) in DIRECTION {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if !map.on_map(nx, ny) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let new_risk = -risk + map.get_val(nx, ny);
            if *seen.entry((nx, ny)).or_insert(i32::MAX) <= new_risk {
                continue;
            }
            max_heap.push((-new_risk, nx, ny));
            *seen.entry((nx, ny)).or_default() = new_risk;
        }
    }
    unreachable!()
}

fn find_lowest_risk_path(input: &str) -> i32 {
    let map = Map::new(parse_input(input), false);
    find_shortest_path(&map)
}

fn find_lowest_risk_path_on_extended_map(input: &str) -> i32 {
    let map = Map::new(parse_input(input), true);
    find_shortest_path(&map)
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", find_lowest_risk_path(&input));
    println!("{}", find_lowest_risk_path_on_extended_map(&input));
}

#[cfg(test)]
mod tests {
    use crate::{find_lowest_risk_path, find_lowest_risk_path_on_extended_map};

    #[test]
    fn test_find_lowest_risk_path() {
        let input = "1163751742
                    1381373672
                    2136511328
                    3694931569
                    7463417111
                    1319128137
                    1359912421
                    3125421639
                    1293138521
                    2311944581";
        assert_eq!(40, find_lowest_risk_path(&input));
    }

    #[test]
    fn test_find_lowest_risk_path_on_extended_map() {
        let input = "1163751742
                    1381373672
                    2136511328
                    3694931569
                    7463417111
                    1319128137
                    1359912421
                    3125421639
                    1293138521
                    2311944581";
        assert_eq!(315, find_lowest_risk_path_on_extended_map(&input));
    }
}
