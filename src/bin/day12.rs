use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

use advent_of_code::read_file_to_string;

fn construct_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    BufReader::new(input.as_bytes()).lines().for_each(|line| {
        let line = line.unwrap();
        let (from, to) = line
            .trim()
            .split_once('-')
            .expect("Expect to have two caves in a line");
        let entry = graph.entry(from.to_owned()).or_default();
        entry.push(to.to_owned());
        let entry = graph.entry(to.to_owned()).or_default();
        entry.push(from.to_owned());
    });
    graph
}

fn explore(at: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, can_revisit: bool) -> u32 {
    if at == "end" {
        return 1;
    }
    let mut added = false;
    if at.chars().all(char::is_lowercase) {
        added = visited.insert(at.into());
    }
    let mut paths = 0;
    for neighbor in graph.get(at).unwrap() {
        if neighbor != "start" && (!visited.contains(neighbor) || can_revisit) {
            // println!("{}: {} going to {}, visited = {:?}, revisit={}", depth, at, neighbor, visited, can_revisit);
            paths += explore(neighbor, graph, visited, can_revisit & !visited.contains(neighbor));
        }
    }
    if added {
        visited.remove(at);
    }
    paths
}

fn count_paths(input: &str, can_revisit_small_cave: bool) -> u32 {
    let graph = construct_graph(input);
    let mut visited = HashSet::new();
    explore("start", &graph, &mut visited, can_revisit_small_cave)
}

fn main() {
    let input = read_file_to_string(env!("CARGO_BIN_NAME"));
    println!("{}", count_paths(&input, false));
    println!("{}", count_paths(&input, true));
}

#[cfg(test)]
mod tests {
    use crate::count_paths;

    #[test]
    fn test_count_paths() {
        let input1 = "start-A
                    start-b
                    A-c
                    A-b
                    b-d
                    A-end
                    b-end";
        assert_eq!(10, count_paths(&input1, false));
        assert_eq!(36, count_paths(&input1, true));
        let input2 = "dc-end
                    HN-start
                    start-kj
                    dc-start
                    dc-HN
                    LN-dc
                    HN-end
                    kj-sa
                    kj-HN
                    kj-dc";
        assert_eq!(19, count_paths(&input2, false));
        assert_eq!(103, count_paths(&input2, true));
        let input3 = "fs-end
                    he-DX
                    fs-he
                    start-DX
                    pj-DX
                    end-zg
                    zg-sl
                    zg-pj
                    pj-he
                    RW-he
                    fs-DX
                    pj-RW
                    zg-RW
                    start-pj
                    he-WI
                    zg-he
                    pj-fs
                    start-RW";
        assert_eq!(226, count_paths(&input3, false));
        assert_eq!(3509, count_paths(&input3, true));
    }
}
