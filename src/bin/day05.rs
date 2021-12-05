use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

use advent_of_code::{read_input, Coordinate, Segment};

trait SegmentMarker {
    fn mark(&self, record: &mut HashMap<(i32, i32), i32>, diagonal: bool);
}

impl SegmentMarker for Segment {
    fn mark(&self, record: &mut HashMap<(i32, i32), i32>, diagonal: bool) {
        let (dx, dy) = self.direction;
        if !diagonal && dx != 0 && dy != 0 {
            return;
        }
        let steps = i32::max(
            i32::abs(self.from.x - self.to.x),
            i32::abs(self.from.y - self.to.y),
        );
        let (x, y) = (self.from.x, self.from.y);
        for i in 0..=steps {
            let count = record.entry((x + i * dx, y + i * dy)).or_default();
            *count += 1;
        }
    }
}

fn get_segment(line: &str) -> Segment {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }
    match RE.captures(line) {
        Some(caps) => Segment::new(
            Coordinate::new(
                caps.name("x1").unwrap().as_str().parse().unwrap(),
                caps.name("y1").unwrap().as_str().parse().unwrap()
            ),
            Coordinate::new(
                caps.name("x2").unwrap().as_str().parse().unwrap(),
                caps.name("y2").unwrap().as_str().parse().unwrap()
            )
        ),
        None => panic!("Cannot parse line {}", line),
    }
}

fn get_overlap_count(input: &Vec<&str>, diagonal: bool) -> usize {
    let segments: Vec<Segment> = input.iter().map(|l| get_segment(l)).collect();
    let mut mark_count = HashMap::new();
    for seg in &segments {
        seg.mark(&mut mark_count, diagonal);
    }
    mark_count.values().filter(|&value| *value > 1).count()
}

fn main() {
    let lines = read_input(env!("CARGO_BIN_NAME"));
    let input = lines.iter().map(|l| l.as_str()).collect();
    println!("{}", get_overlap_count(&input, false));
    println!("{}", get_overlap_count(&input, true));
}

#[cfg(test)]
mod tests {
    use crate::get_overlap_count;

    #[test]
    fn test_get_overlap_count() {
        let data = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];
        assert_eq!(5, get_overlap_count(&data, false));
        assert_eq!(12, get_overlap_count(&data, true));
    }
}
