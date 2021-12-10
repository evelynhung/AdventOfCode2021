use std::io::{BufRead, BufReader};

use advent_of_code::read_file_to_string;

enum ErrorType {
    Corrupted,
    Incomplete,
}

fn get_paired(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn syntax_check(line: &str) -> (ErrorType, Vec<char>) {
    let mut stack: Vec<char> = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ch => match stack.pop() {
                Some(top) if top == get_paired(ch) => {}
                _ => {
                    return (ErrorType::Corrupted, vec![ch]);
                }
            },
        }
    }
    match stack.is_empty() {
        true => panic!("We shouldn't have a perfect match. line = {}", line),
        false => (ErrorType::Incomplete, stack),
    }
}

fn calc_corrupted_points(input: &str) -> u32 {
    BufReader::new(input.as_bytes())
        .lines()
        .map(|line| match syntax_check(line.unwrap().trim()) {
            (ErrorType::Corrupted, mismatch) => match mismatch[0] {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            },
            _ => 0,
        })
        .sum()
}

fn calc_incomplete_points(input: &str) -> u128 {
    let mut scores: Vec<u128> = BufReader::new(input.as_bytes())
        .lines()
        .map(|line| match syntax_check(line.unwrap().trim()) {
            (ErrorType::Incomplete, mismatch) => mismatch
                .iter()
                .rev()
                .fold(0, |sum, ch| sum * 5 + match get_paired(*ch) {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                }),
            _ => 0,
        })
        .filter(|score| *score > 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len()/2]
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", calc_corrupted_points(&input));
    println!("{}", calc_incomplete_points(&input));
}

#[cfg(test)]
mod tests {
    use crate::{calc_corrupted_points, calc_incomplete_points};

    #[test]
    fn test_calc_corrupted_points() {
        let input = "[({(<(())[]>[[{[]{<()<>>
                    [(()[<>])]({[<{<<[]>>(
                    {([(<{}[<>[]}>{[]{[(<()>
                    (((({<>}<{<{<>}{[]{[]{}
                    [[<[([]))<([[{}[[()]]]
                    [{[{({}]{}}([{[{{{}}([]
                    {<[[]]>}<{[{[{[]{()[[[]
                    [<(<(<(<{}))><([]([]()
                    <{([([[(<>()){}]>(<<{{
                    <{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(26397, calc_corrupted_points(&input))
    }

    #[test]
    fn test_calc_incomplete_points() {
        let input = "[({(<(())[]>[[{[]{<()<>>
                    [(()[<>])]({[<{<<[]>>(
                    {([(<{}[<>[]}>{[]{[(<()>
                    (((({<>}<{<{<>}{[]{[]{}
                    [[<[([]))<([[{}[[()]]]
                    [{[{({}]{}}([{[{{{}}([]
                    {<[[]]>}<{[{[{[]{()[[[]
                    [<(<(<(<{}))><([]([]()
                    <{([([[(<>()){}]>(<<{{
                    <{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(288957, calc_incomplete_points(&input))
    }
}
