use std::{
    collections::{HashMap, VecDeque},
    io::{BufRead, BufReader},
};

use advent_of_code::read_file_to_string;

const DIRECTIONS: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn on_map(x: i32, y: i32, rows: usize, cols: usize) -> bool {
    x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32
}

fn is_valley(height_map: &Vec<Vec<u32>>, x: usize, y: usize, rows: usize, cols: usize) -> bool {
    for (dx, dy) in DIRECTIONS {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if !on_map(nx, ny, rows, cols) {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if height_map[x][y] >= height_map[nx][ny] {
            return false;
        }
    }
    true
}

fn get_height_map(input: &str) -> Vec<Vec<u32>> {
    BufReader::new(input.as_bytes())
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .chars()
                .map(|c| c.to_digit(10).expect("input should be 0-9 digit"))
                .collect()
        })
        .collect()
}

fn calc_risk_of_low_points(input: &str) -> i32 {
    let height_map = get_height_map(input);

    let (rows, cols) = (height_map.len(), height_map[0].len());
    let mut risk: i32 = 0;
    for i in 0..rows {
        for j in 0..cols {
            if is_valley(&height_map, i, j, rows, cols) {
                risk += (1 + height_map[i][j]) as i32;
            }
        }
    }
    risk
}

fn explore_basin(
    height_map: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    visited: &mut HashMap<(usize, usize), usize>,
    id: usize,
) -> i32 {
    
    let (rows, cols) = (height_map.len(), height_map[0].len());
    let mut queue = VecDeque::new();
    let mut size = 0;
    queue.push_back((i, j));
    visited.insert((i, j), id);

    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();
        size += 1;
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if !on_map(nx, ny, rows, cols) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if height_map[nx][ny] != 9 {
                if !visited.contains_key(&(nx, ny)) {
                    visited.insert((nx, ny), id);
                    queue.push_back((nx, ny));
                } else if *visited.get(&(nx, ny)).unwrap() != id {
                    panic!("a cell belongs to two basins");
                }
            }
        }
    }
    size
}

fn calc_top3_basin(input: &str) -> i32 {
    let height_map = get_height_map(input);
    let (rows, cols) = (height_map.len(), height_map[0].len());
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut basin_size: Vec<i32> = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if is_valley(&height_map, i, j, rows, cols) {
                let size = explore_basin(&height_map, i, j, &mut visited, basin_size.len());
                basin_size.push(size);
            }
        }
    }
    basin_size.sort();
    basin_size.reverse();
    basin_size[0] * basin_size[1] * basin_size[2]
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", calc_risk_of_low_points(&input));
    println!("{}", calc_top3_basin(&input));
}

#[cfg(test)]
mod tests {
    use crate::{calc_risk_of_low_points, calc_top3_basin};

    #[test]
    fn test_calc_risk_of_low_points() {
        let input = "2199943210
                    3987894921
                    9856789892
                    8767896789
                    9899965678";
        assert_eq!(15, calc_risk_of_low_points(&input));
    }

    #[test]
    fn test_calc_top3_basin() {
        let input = "2199943210
                    3987894921
                    9856789892
                    8767896789
                    9899965678";
        assert_eq!(1134, calc_top3_basin(&input));
    }
}
