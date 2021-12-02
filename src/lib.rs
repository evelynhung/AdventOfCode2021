use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(filename: &str) -> Vec<String> {
    let data_folder = "data";
    let input_file_path = format!("{}/{}.txt", data_folder, filename);

    let file = match File::open(&input_file_path) {
        Ok(file) => file,
        Err(why) => panic!("Can't not open {}: {}", input_file_path, why),
    };
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}
