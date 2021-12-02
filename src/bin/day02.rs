use std::{num::ParseIntError, str::FromStr};

use advent_of_code::read_input;

enum Operation {
    Forward(i32),
    Up(i32),
    Down(i32),
}

struct Position {
    x: i32,
    z: i32,
}

impl Position {
    fn new(x: i32, z: i32) -> Position {
        Position { x, z }
    }
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let direction = split.next().unwrap();
        let unit = split.next().unwrap().parse::<i32>()?;
        Ok(match direction {
            "forward" => Operation::Forward(unit),
            "up" => Operation::Up(unit),
            "down" => Operation::Down(unit),
            _ => panic!("Can't translate to operation {} ", direction),
        })
    }
}

fn summed_moves(operations: &Vec<Operation>) -> Position {
    let mut x = 0;
    let mut z = 0;
    for op in operations {
        match op {
            Operation::Forward(unit) => x += unit,
            Operation::Down(unit) => z += unit,
            Operation::Up(unit) => z -= unit,
        }
    }

    Position::new(x, z)
}

fn aimed_moves(operations: &Vec<Operation>) -> Position {
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;

    for op in operations {
        match op {
            Operation::Forward(unit) => {
                x += unit;
                z += unit * aim
            }
            Operation::Down(unit) => aim += unit,
            Operation::Up(unit) => aim -= unit,
        }
    }

    Position::new(x, z)
}

fn get_operations(input: Vec<&str>) -> Vec<Operation> {
    input.iter().map(|line| line.parse().unwrap()).collect()
}

fn main() {
    let lines = read_input(env!("CARGO_BIN_NAME"));
    let input = lines.iter().map(|l| l.as_str()).collect();
    let operations = get_operations(input);
    let summed_position = summed_moves(&operations);
    let aimed_position = aimed_moves(&operations);
    println!("{}", summed_position.x * summed_position.z);
    println!("{}", aimed_position.x * aimed_position.z);
}

#[cfg(test)]
mod tests {
    use crate::{aimed_moves, get_operations, summed_moves};

    #[test]
    fn test_summed_moves() {
        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let operations = get_operations(data);
        let position = summed_moves(&operations);
        assert_eq!(15, position.x);
        assert_eq!(10, position.z);
    }

    #[test]
    fn test_aimed_moves() {
        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let operations = get_operations(data);
        let position = aimed_moves(&operations);
        assert_eq!(15, position.x);
        assert_eq!(60, position.z);
    }
}
