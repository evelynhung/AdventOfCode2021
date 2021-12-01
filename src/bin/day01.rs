use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<i32> {
    let data_folder = "data";
    let input_file_path = format!("{}/{}.txt", data_folder, env!("CARGO_BIN_NAME"));

    let file = match File::open(&input_file_path) {
        Ok(file) => file,
        Err(why) => panic!("Can't not open {}: {}", input_file_path, why),
    };
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

fn measure(numbers: &[i32], window_size: usize) -> i32 {
    let mut counter = 0;
    let mut window_sum: i32 = numbers[..window_size].iter().sum();
    for idx in window_size..numbers.len() {
        let pre_window_sum = window_sum;
        window_sum += numbers[idx] - numbers[idx - window_size];
        if window_sum > pre_window_sum {
            counter += 1;
        }
    }
    counter
}
fn main() {
    let numbers = read_input();
    println!("{}", measure(&numbers, 1)); // Part 1
    println!("{}", measure(&numbers, 3)); // Part 2
}

#[cfg(test)]
mod tests {
    use crate::measure;

    #[test]
    fn test_measure() {
        assert_eq!(
            7,
            measure(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 1)
        );
        assert_eq!(
            5,
            measure(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 3)
        );
    }
}
