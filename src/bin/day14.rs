use itertools::Itertools;
use std::collections::HashMap;

use advent_of_code::read_file_to_string;

type Pair = (char, char);

fn parse_input(input: &str) -> (HashMap<Pair, u64>, HashMap<Pair, char>) {
    let (template_str, rules_str) = input
        .split_once("\n\n")
        .expect("Input should have template and rules");

    let mut template: HashMap<Pair, u64> = HashMap::new();
    template_str
        .chars()
        .tuple_windows()
        .for_each(|(c1, c2)| *template.entry((c1, c2)).or_default() += 1);

    let rules: HashMap<Pair, char> = rules_str
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().split_once("->").unwrap())
        .map(|(from_str, to_str)| {
            let mut from_it = from_str.trim().chars();
            let mut to_it = to_str.trim().chars();
            (
                (from_it.next().unwrap(), from_it.next().unwrap()),
                to_it.next().unwrap(),
            )
        })
        .collect();

    (template, rules)
}

fn grow_polymer(template: &HashMap<Pair, u64>, rules: &HashMap<Pair, char>, times: u32) -> HashMap<Pair, u64> {
    let mut template = template.to_owned();
    for _ in 0..times {
        let mut generated: HashMap<(char, char), u64> = HashMap::new();
        template.iter().for_each(|((c1, c2), count)| {
            let element = *rules.get(&(*c1, *c2)).expect("should covered by rule");
            *generated.entry((*c1, element)).or_default() += count;
            *generated.entry((element, *c2)).or_default() += count;
        });
        template = generated;
    }
    template
}

fn element_count(template: &HashMap<Pair, u64>) -> HashMap<char, u64> {
    let mut element_count: HashMap<char, u64> = HashMap::new();
    template.iter().for_each(|((c1, c2), count)| {
        *element_count.entry(*c1).or_default() += count;
        *element_count.entry(*c2).or_default() += count;
    });
    element_count
}

fn count_quantity_diff_from_generated_polymer(input: &str, times: u32) -> u64 {
    let (template, rules) = parse_input(input);
    let resulted_polymer = grow_polymer(&template, &rules, times);
    let count = element_count(&resulted_polymer);
    let most_element = count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let least_element = count.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    
    (most_element.1 + 1)/2 - (least_element.1 + 1)/2
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", count_quantity_diff_from_generated_polymer(&input, 10));
    println!("{}", count_quantity_diff_from_generated_polymer(&input, 40));
}

#[cfg(test)]
mod tests {
    use crate::{count_quantity_diff_from_generated_polymer};

    #[test]
    fn test_count_quantity_diff_from_generated_polymer() {
        let input = "NNCB

                    CH -> B
                    HH -> N
                    CB -> H
                    NH -> C
                    HB -> C
                    HC -> B
                    HN -> C
                    NN -> C
                    BH -> H
                    NC -> B
                    NB -> B
                    BN -> B
                    BB -> N
                    BC -> B
                    CC -> N
                    CN -> C";
        assert_eq!(1588, count_quantity_diff_from_generated_polymer(&input, 10));
        assert_eq!(2188189693529, count_quantity_diff_from_generated_polymer(&input, 40));
    }
}
