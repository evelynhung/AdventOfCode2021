use std::collections::VecDeque;

use advent_of_code::read_file_to_string;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|d| d.to_digit(10).expect("energy level should be from 0-9"))
                .collect()
        })
        .collect()
}

fn on_map(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    x >= 0 && y >= 0 && x < rows as isize && y < cols as isize
}

fn increase_and_check_energy(grid: &mut Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    (grid[i][j] <= 9)
        .then(|| {
            grid[i][j] += 1;
            grid[i][j] > 9
        })
        .unwrap_or(false)
}

fn step_routine(grid: &mut Vec<Vec<u32>>) -> u32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..rows {
        for j in 0..cols {
            if increase_and_check_energy(grid, i, j) {
                queue.push_back((i, j));
            }
        }
    }

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if !on_map(nx, ny, rows, cols) {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if increase_and_check_energy(grid, nx, ny) {
                queue.push_back((nx, ny));
            }
        }
    }
    
    let mut flashes = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] > 9 {
                grid[i][j] = 0;
                flashes += 1;
            }
        }
    }
    flashes
}

fn calc_flashes(input: &str, steps: u32) -> u32 {
    let mut grid = get_grid(input);
    (0..steps).map(|_| step_routine(&mut grid)).sum()
}

fn calc_synchronizing_flash(input: &str) -> u32 {
    let mut grid = get_grid(input);
    let total_octopuses = (grid.len() * grid[0].len()) as u32;
    for step in 1..u32::MAX {
        if step_routine(&mut grid) == total_octopuses {
            return step;
        }
    }
    unreachable!("We should be able to find a step with all octopuses lighting up");
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", calc_flashes(&input, 100));
    println!("{}", calc_synchronizing_flash(&input));
}

#[cfg(test)]
mod tests {
    use crate::{calc_flashes, calc_synchronizing_flash};

    #[test]
    fn test_calc_flashes() {
        let input = "5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526";
        assert_eq!(1656, calc_flashes(&input, 100));
    }

    #[test]
    fn test_calc_synchronizing_flash() {
        let input = "5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526";
        assert_eq!(195, calc_synchronizing_flash(&input));
    }
}
