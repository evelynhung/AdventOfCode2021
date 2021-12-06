use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

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

pub fn read_file_to_string(filename: &str) -> String {
    let data_folder = "data";
    let input_file_path = format!("{}/{}.txt", data_folder, filename);

    let mut file = match File::open(&input_file_path) {
        Ok(file) => file,
        Err(why) => panic!("Can't not open {}: {}", input_file_path, why),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Can't read input file to string.");

    buf
}

pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

pub struct Segment {
    pub from: Coordinate,
    pub to: Coordinate,
    pub direction: (i32, i32),
}

impl Segment {
    pub fn new(from: Coordinate, to: Coordinate) -> Segment {
        let direction = (compare_to_int(from.x, to.x), compare_to_int(from.y, to.y));
        Segment { from, to, direction }
    }
}

pub fn compare_to_int(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}
