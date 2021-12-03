use advent_of_code::read_input;

fn count_bits(bytes_numbers: &Vec<Vec<u8>>, index: usize) -> i32 {
    let mut counter = 0;
    for bytes in bytes_numbers {
        counter += match bytes[index] {
            b'1' => 1,
            b'0' => -1,
            _ => 0,
        };
    }
    counter
}

fn to_bytes_numbers(input: &Vec<&str>) -> Vec<Vec<u8>> {
    input.iter().map(|&s| s.to_owned().into_bytes()).collect()
}

fn diagnose_power_consumption(input: &Vec<&str>) -> (i32, i32) {
    let bit_length = input[0].len();
    let bytes_numbers = to_bytes_numbers(input);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..bit_length {
        gamma <<= 1;
        epsilon <<= 1;
        match count_bits(&bytes_numbers, i) {
            count if count > 0 => gamma += 1,
            count if count < 0 => epsilon += 1,
            _ => panic!("equally common at bit_{}", i),
        }
    }
    (gamma, epsilon)
}

fn get_most_common_value(bytes_numbers: &Vec<Vec<u8>>, index: usize) -> u8 {
    match count_bits(bytes_numbers, index) {
        count if count > 0 => b'1',
        count if count < 0 => b'0',
        _ => b'1',
    }
}

fn get_least_common_value(bytes_numbers: &Vec<Vec<u8>>, index: usize) -> u8 {
    match count_bits(bytes_numbers, index) {
        count if count > 0 => b'0',
        count if count < 0 => b'1',
        _ => b'0',
    }
}

type CriteriaFn = dyn Fn(&Vec<Vec<u8>>, usize) -> u8;

fn filter_by_criteria(bytes_numbers: &Vec<Vec<u8>>, bit_length: usize, criteria: &CriteriaFn) -> Vec<u8> {
    let mut bytes_numbers = bytes_numbers.to_owned();
    for index in 0..bit_length {
        let value = criteria(&bytes_numbers, index);
        bytes_numbers = bytes_numbers
            .iter()
            .map(|bytes| bytes.to_owned())
            .filter(|bytes| bytes[index] == value)
            .collect();
        if bytes_numbers.len() == 1 {
            break;
        }
    }
    bytes_numbers[0].clone()
}

fn bytes_to_i32(bytes: Vec<u8>) -> i32 {
    let mut num = 0;
    for b in bytes {
        num <<= 1;
        num += (b - b'0') as i32;
    }
    num
}

fn diagnose_life_support(input: &Vec<&str>) -> (i32, i32) {
    let bit_length = input[0].len();
    let bytes_numbers = to_bytes_numbers(input);

    let oxygen = filter_by_criteria(&bytes_numbers, bit_length, &get_most_common_value);
    let co2 = filter_by_criteria(&bytes_numbers, bit_length, &get_least_common_value);

    (bytes_to_i32(oxygen), bytes_to_i32(co2))
}

fn main() {
    let lines = read_input(env!("CARGO_BIN_NAME"));
    let input = lines.iter().map(|l| l.as_str()).collect();
    let (gamma, epsilon) = diagnose_power_consumption(&input);
    let (oxygen, co2) = diagnose_life_support(&input);
    println!("{}", gamma * epsilon);
    println!("{}", oxygen * co2);
}

#[cfg(test)]
mod tests {
    use crate::{diagnose_life_support, diagnose_power_consumption};

    #[test]
    fn test_diagnose_power_consumption() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let (gamma, epsilon) = diagnose_power_consumption(&data);
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon);
    }

    #[test]
    fn test_diagnose_life_support() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let (oxygen, co2) = diagnose_life_support(&data);
        assert_eq!(23, oxygen);
        assert_eq!(10, co2);
    }
}
