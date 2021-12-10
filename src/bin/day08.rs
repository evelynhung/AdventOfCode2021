use std::{
    collections::HashMap,
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

fn parse_entry(entry: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, output_digits) = entry.split_once('|').expect("Where is my IO?");
    (parse_digits(patterns.trim()), parse_digits(output_digits.trim()))
}

fn count_uqique_digits(input: &str) -> usize {
    BufReader::new(input.as_bytes())
        .lines()
        .flat_map(|entry| parse_entry(&entry.unwrap()).1)
        .filter(|digit_str| matches!(digit_str.len(), 2 | 3 | 4 | 7))
        .count()
}

fn reasoning_digits(patterns: &[String]) -> HashMap<String, u8> {
    // Use "one" and "four" as filters for (2, 3, 5) and (0, 6, 9)
    let one = patterns.iter().find(|ptn| ptn.len() == 2).expect("There should have an ONE");
    let four = patterns.iter().find(|ptn| ptn.len() == 4).expect("There should have a FOUR");
    patterns
        .iter()
        .map(|ptn| {
            let pattern = ptn.to_owned();
            match pattern.len() {
                2 => (pattern, 1),
                3 => (pattern, 7),
                4 => (pattern, 4),
                7 => (pattern, 8),
                len => match (
                    len,
                    pattern.chars().filter(|c| one.contains(*c)).count(),
                    pattern.chars().filter(|c| four.contains(*c)).count(),
                ) {
                    (5, 1, 2) => (pattern, 2),
                    (5, 2, 3) => (pattern, 3),
                    (5, 1, 3) => (pattern, 5),
                    (6, 2, 3) => (pattern, 0),
                    (6, 1, 3) => (pattern, 6),
                    (6, 2, 4) => (pattern, 9),
                    _ => unreachable!(),
                },
            }
        })
        .collect()
}

fn get_output_digits(entry: &str) -> u32 {
    let (patterns, output_digits) = parse_entry(entry);

    let pattern_to_digits = reasoning_digits(&patterns);

    output_digits
        .iter()
        .map(|pattern| {
            pattern_to_digits
                .get(pattern)
                .expect("A mapping from pattern to digit should exist")
                .to_owned()
        })
        .enumerate()
        .fold(0, |sum, (i, d)| sum + d as u32 * 10u32.pow(3 - i as u32))
}

fn addup_output_digits(input: &str) -> u32 {
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
        assert_eq!(5353, get_output_digits(&data));
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
            assert_eq!(ans_set[i], get_output_digits(&data_set[i]))
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
