use advent_of_code::read_file_to_string;

fn get_crab_positions(input: &str) -> Vec<i32> {
    input
        .split(',')
        .filter(|token| !token.trim().is_empty())
        .map(|token| token.parse().unwrap())
        .collect()
}

fn find_median(positions: &mut [i32]) -> i32 {
    positions.sort_unstable();

    let mid = positions.len() / 2;
    positions[mid]
}

fn calc_distance_sum(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|&val| i32::abs(val - target)).sum()
}

fn find_average_floor(positions: &[i32]) -> i32 {
    let avg = positions.iter().sum::<i32>() as f64 / positions.len() as f64;
    avg.floor() as i32
}

fn calc_weighted_distance_sum(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|&pos| {
            let distance = i32::abs(pos - target);
            distance * (distance + 1) / 2
        })
        .sum()
}

fn align_crabs(input: &str) -> i32 {
    let mut positions = get_crab_positions(input);
    let median = find_median(&mut positions);
    calc_distance_sum(&positions, median)
}

fn weighted_align_crabs(input: &str) -> i32 {
    let positions = get_crab_positions(input);
    let avg = find_average_floor(&positions);
    i32::min(
        calc_weighted_distance_sum(&positions, avg),
        calc_weighted_distance_sum(&positions, avg + 1),
    )
}

fn main() {
    let mut input = read_file_to_string(env!("CARGO_BIN_NAME"));
    input.pop(); // discard newline
    println!("{}", align_crabs(&input));
    println!("{}", weighted_align_crabs(&input));
}

#[cfg(test)]
mod tests {
    use crate::{align_crabs, weighted_align_crabs};

    #[test]
    fn test_align_crabs() {
        let data = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(37, align_crabs(&data));
        assert_eq!(168, weighted_align_crabs(&data));
    }
}
