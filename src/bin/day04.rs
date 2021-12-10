use advent_of_code::read_file_to_string;
use std::collections::HashMap;

struct Board {
    num_to_index: HashMap<u8, usize>,
    h_lines: [u8; 5],
    v_lines: [u8; 5],
    remain_sum: i32,
    done: bool,
}

impl Board {
    fn new(data: &[u8]) -> Board {
        let num_to_index: HashMap<u8, usize> =
            data.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        let v_lines = [0; 5];
        let h_lines = [0; 5];
        let remain_sum: i32 = data.iter().map(|&n| n as i32).sum();
        Board {
            num_to_index,
            v_lines,
            h_lines,
            remain_sum,
            done: false,
        }
    }
    fn check_and_mark(&mut self, num: u8) -> bool {
        if !self.num_to_index.contains_key(&num) {
            return false;
        }
        let index = self.num_to_index.get(&num).unwrap();

        let (i, j) = (index / 5, index % 5);
        self.h_lines[i] += 1;
        self.v_lines[j] += 1;
        self.remain_sum -= num as i32;

        if self.h_lines[i] == 5 || self.v_lines[j] == 5 {
            self.done = true;
        }

        self.done
    }
}

fn read_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let (num_str, boards_str) = input.split_once("\n\n").unwrap();
    let picked_nums: Vec<u8> = num_str.trim()
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = vec![];
    boards_str.split("\n\n").for_each(|b_str| {
        let board: Vec<u8> = b_str
            .split_ascii_whitespace()
            .map(|token| token.parse().unwrap())
            .collect();
        
        boards.push(Board::new(&board));
    });

    (picked_nums, boards)
}

fn bingo_to_win(filename: &str) -> i32 {
    let (picked_nums, mut boards) = read_input(filename);
    for num in picked_nums {
        for b in &mut boards {
            if b.check_and_mark(num) {
                return b.remain_sum * (num as i32);
            }
        }
    }
    unreachable!("Must have a winner");
}

fn bingo_to_lose(filename: &str) -> i32 {
    let (picked_nums, mut boards) = read_input(filename);
    let mut completed = boards.len();
    let mut score = 0;
    for num in picked_nums {
        for board in &mut boards {
            if !board.done && board.check_and_mark(num) {
                completed -= 1;
            }
            if completed == 0 {
                score = board.remain_sum * (num as i32);
                break;
            }
        }
        if score > 0 {
            return score;
        }
    }
    unreachable!("Must have a loser");
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    let win_score = bingo_to_win(&input);
    let lose_score = bingo_to_lose(&input);
    println!("{}", win_score);
    println!("{}", lose_score);
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_file_to_string;

    use crate::{bingo_to_lose, bingo_to_win};

    #[test]
    fn test_bingo_to_win() {
        let data = read_file_to_string("day04-small");
        let score = bingo_to_win(&data);
        assert_eq!(score, 4512);
    }

    #[test]
    fn test_bingo_to_lose() {
        let data = read_file_to_string("day04-small");
        let score = bingo_to_lose(&data);
        assert_eq!(score, 1924);
    }
}
