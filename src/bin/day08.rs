use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

use advent_of_code::read_file_to_string;

fn parse_digits(pattern: &str) -> Vec<String> {
    pattern
        .split(' ')
        .map(|token| {
            let mut chars: Vec<char> = token.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            chars.iter().collect()
        })
        .collect()
}

fn parse_entry(entry: &String) -> (Vec<String>, Vec<String>) {
    let mut split = entry.split('|');
    let patterns = split.next().expect("should have input patterns").trim();
    let output_digits = split.next().expect("should have output digits").trim();

    (parse_digits(patterns), parse_digits(output_digits))
}

fn count_uqique_digits(input: &str) -> i32 {
    BufReader::new(input.as_bytes())
        .lines()
        .flat_map(|entry| parse_entry(&entry.unwrap()).1)
        .map(|digit_str| match digit_str.len() {
            2 | 3 | 4 | 7 => 1,
            _ => 0,
        })
        .sum()
}

fn find_diff_char_count(ptn1: &str, ptn2: &str) -> usize {
    let ptn_set1: HashSet<char> = ptn1.chars().collect();
    let ptn_set2: HashSet<char> = ptn2.chars().collect();
    ptn_set1.symmetric_difference(&ptn_set2).count()
}

fn reasoning_digits(patterns: &Vec<String>) -> HashMap<String, String> {
    let mut len_to_patterns: HashMap<usize, Vec<String>> = HashMap::new();
    let mut pattern_to_digit = HashMap::new();
    for pattern in patterns {
        let length = pattern.len();
        len_to_patterns
            .entry(length)
            .and_modify(|e| e.push(pattern.to_owned()))
            .or_insert(vec![pattern.to_owned()]);

        match length {
            2 => {
                pattern_to_digit.insert(pattern.to_owned(), "1".into());
            }
            3 => {
                pattern_to_digit.insert(pattern.to_owned(), "7".into());
            }
            4 => {
                pattern_to_digit.insert(pattern.to_owned(), "4".into());
            }
            7 => {
                pattern_to_digit.insert(pattern.to_owned(), "8".into());
            }
            _ => {}
        }
    }
    let len5_group = len_to_patterns.get(&5).unwrap();
    let len6_group = len_to_patterns.get(&6).unwrap();
    for len5_item in len5_group {
        let sum_diff: usize = len6_group
            .iter()
            .map(|len6_item| find_diff_char_count(len5_item, len6_item))
            .sum();
        match sum_diff {
            9 => {
                // 2's diff to {0, 6, 9} = 3 + 3 + 3
                pattern_to_digit.insert(len5_item.to_owned(), "2".into());
            }
            5 => {
                // 5's diff to {0, 6, 9} = 3 + 1 + 1
                pattern_to_digit.insert(len5_item.to_owned(), "5".into());
            }
            7 => {
                // 3's diff to {0, 6, 9} = 3 + 3 + 1
                pattern_to_digit.insert(len5_item.to_owned(), "3".into());
            }
            _ => {}
        }
    }

    for len6_item in len6_group {
        let sum_diff: usize = len5_group
            .iter()
            .map(|len5_item| find_diff_char_count(len6_item, len5_item))
            .sum();
        match sum_diff {
            9 => {
                // 0's diff to {2, 3, 5} = 3 + 3 + 3
                pattern_to_digit.insert(len6_item.to_owned(), "0".into());
            }
            5 => {
                // 9's diff to {2, 3, 5} = 3 + 1 + 1
                pattern_to_digit.insert(len6_item.to_owned(), "9".into());
            }
            7 => {
                // 6's diff to {2, 3, 5} = 3 + 3 + 1
                pattern_to_digit.insert(len6_item.to_owned(), "6".into());
            }
            _ => {}
        }
    }

    pattern_to_digit
}

fn get_output_digits(entry: &String) -> i32 {
    let (patterns, output_digits) = parse_entry(entry);

    let pattern_to_digits = reasoning_digits(&patterns);

    let digits: Vec<String> = output_digits
        .iter()
        .map(|pattern| {
            pattern_to_digits
                .get(pattern)
                .expect("A mapping from pattern to digit should exist")
                .to_owned()
        })
        .collect();
    digits.join("").parse().unwrap()
}

fn addup_output_digits(input: &str) -> i32 {
    let input: Vec<String> = BufReader::new(input.as_bytes())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    input.iter().map(|entry| get_output_digits(entry)).sum()
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", count_uqique_digits(&input));
    println!("{}", addup_output_digits(&input));
}

#[cfg(test)]
mod tests {
    use crate::{addup_output_digits, count_uqique_digits, get_output_digits};

    #[test]
    fn test_count_uqique_digits() {
        let data =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(26, count_uqique_digits(&data));
    }

    #[test]
    fn test_get_output_digits() {
        let data =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, get_output_digits(&data.into()));
        let data_set = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];
        let ans_set = vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];
        for i in 0..data_set.len() {
            assert_eq!(ans_set[i], get_output_digits(&data_set[i].into()))
        }
    }

    #[test]
    fn test_addup_output_digits() {
        let data =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(61229, addup_output_digits(&data));
    }
}
