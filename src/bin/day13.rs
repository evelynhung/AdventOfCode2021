#![feature(hash_drain_filter)]
use std::collections::HashSet;

use advent_of_code::read_file_to_string;

struct OrigamiPaper {
    dots: HashSet<(u32, u32)>,
    folds: Vec<(u32, u32)>,
    fold_times: usize,
}

impl OrigamiPaper {
    fn new(dots: HashSet<(u32, u32)>, folds: Vec<(u32, u32)>) -> OrigamiPaper {
        OrigamiPaper { dots, folds, fold_times: 0 }
    }

    fn fold(&mut self) -> bool {
        let (fx, fy) = self.folds[self.fold_times];
        let to_align: HashSet<(u32, u32)> = self.dots.drain_filter(|(x, y)| x >= &fx && y >= &fy).collect();
        to_align.iter().for_each(|(x, y)| match (fx, fy) {
            (0, pos) => {
                self.dots.insert((*x, pos - (*y - pos)));
            }
            (pos, 0) => {
                self.dots.insert((pos - (*x - pos), *y));
            }
            _ => unreachable!(),
        });
        self.fold_times += 1;

        self.fold_times < self.folds.len()
    }
}

fn get_dot_locations(input: &str) -> HashSet<(u32, u32)> {
    input
        .split('\n')
        .map(|line| line.trim().split_once(',').unwrap())
        .map(|(x_str, y_str)| (x_str.parse::<u32>().unwrap(), y_str.parse::<u32>().unwrap()))
        .collect()
}

fn get_folds(input: &str) -> Vec<(u32, u32)> {
    input
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once('=').unwrap())
        .map(|(dir_str, pos_str)| {
            let pos = pos_str.parse::<u32>().unwrap();
            match dir_str.bytes().last().unwrap() {
                b'x' => (pos, 0),
                b'y' => (0, pos),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn get_origami_paper(input: &str) -> OrigamiPaper {
    let (dots, folds) = input.split_once("\n\n").unwrap();

    OrigamiPaper::new(get_dot_locations(dots), get_folds(folds))
}

fn print(paper: &OrigamiPaper) {
    let mut dots: Vec<(u32, u32)> = paper.dots.iter().map(|(x, y)| (*y, *x)).collect();
    dots.sort_unstable();
    let rows = dots.iter().map(|(x, _)| x).max().unwrap().to_owned();
    let cols = dots.iter().map(|(_, y)| y).max().unwrap().to_owned();
    let mut k = 0;
    for i in 0..=rows {
        for j in 0..=cols {
            let mut symbol = ' ';
            if dots[k] == (i, j) {
                symbol = '#';
                k += 1;
            }
            print!("{}", symbol);
        }
        println!();
    }
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    let mut origami_paper = get_origami_paper(&input);
    origami_paper.fold();
    println!("{}", origami_paper.dots.len());
    while origami_paper.fold() {}
    print(&origami_paper);
}

#[cfg(test)]
mod tests {
    use crate::{get_origami_paper, print};

    #[test]
    fn test_fold_paper() {
        let input = "6,10
                    0,14
                    9,10
                    0,3
                    10,4
                    4,11
                    6,0
                    6,12
                    4,1
                    0,13
                    10,12
                    3,4
                    3,0
                    8,4
                    1,10
                    2,14
                    8,10
                    9,0

                    fold along y=7
                    fold along x=5";
        let mut origami_paper = get_origami_paper(&input);
        origami_paper.fold();
        println!("{}", origami_paper.dots.len());
        while origami_paper.fold() { }
        print(&origami_paper);
    }
}
