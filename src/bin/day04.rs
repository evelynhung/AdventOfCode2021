use std::{collections::HashMap, fs::File, io::Read};
use text_io::{read, try_read};

struct Board {
    num_to_index: HashMap<u8, usize>,
    h_lines: [u8; 5],
    v_lines: [u8; 5],
    remain_sum: i32,
    done: bool,
}

impl Board {
    fn new(data: &[u8; 25]) -> Board {
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

fn read_input(filename: &str) -> (Vec<u8>, Vec<Board>) {
    let data_folder = "data";
    let input_file_path = format!("{}/{}.txt", data_folder, filename);

    let mut file_bytes = match File::open(&input_file_path) {
        Ok(file) => file.bytes().map(|ch| ch.unwrap()),
        Err(why) => panic!("Can't not open {}: {}", input_file_path, why),
    };

    let line: String = read!("{}\n", file_bytes);
    let picked_nums: Vec<u8> = line
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = vec![];
    loop {
        let mut board = [0; 25];
        let ret: Result<u8, _> = try_read!("{}", file_bytes);
        if ret.is_err() {
            break;
        }
        board[0] = ret.unwrap();
        for cell in board.iter_mut().skip(1) {
            *cell = read!("{}", file_bytes);
        }
        boards.push(Board::new(&board));
    }

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
    let win_score = bingo_to_win(env!("CARGO_BIN_NAME"));
    let lose_score = bingo_to_lose(env!("CARGO_BIN_NAME"));
    println!("{}", win_score);
    println!("{}", lose_score);
}

#[cfg(test)]
mod tests {
    use crate::{bingo_to_lose, bingo_to_win};

    #[test]
    fn test_bingo_to_win() {
        let score = bingo_to_win("day04-small");
        assert_eq!(score, 4512);
    }

    #[test]
    fn test_bingo_to_lose() {
        let score = bingo_to_lose("day04-small");
        assert_eq!(score, 1924);
    }
}
