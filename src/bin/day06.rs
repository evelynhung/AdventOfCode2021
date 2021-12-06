use advent_of_code::read_file_to_string;

const GROW_UP_DAYS: usize = 9;
const BIRTH_PERIOD: usize = 7;

fn count_laternfish_children(life_days: usize, children_count: &[i128]) -> i128 {
    let mut count: i128 = 0;

    let mut child_life_days = life_days - GROW_UP_DAYS;
    // count the child and its children
    count += 1 + children_count[child_life_days];
    while child_life_days >= BIRTH_PERIOD {
        child_life_days -= BIRTH_PERIOD;
        count += 1 + children_count[child_life_days];
    }
    count
}

fn count_laternfish(input: &str, world_days: usize) -> i128 {
    // children_count is a DP table
    let max_live_days = world_days + GROW_UP_DAYS;
    let mut children_count = vec![0i128; max_live_days];
    for life_days in GROW_UP_DAYS..max_live_days {
        children_count[life_days] = count_laternfish_children(life_days, &children_count);
    }

    input
        .split(',')
        .filter(|token| !token.trim().is_empty())
        .map(|token| token.parse::<usize>().unwrap())
        .map(|value| GROW_UP_DAYS - (value + 1) + world_days)
        .map(|life_days| 1 + children_count[life_days])
        .sum::<i128>()
}

fn main() {
    let mut input = read_file_to_string(env!("CARGO_BIN_NAME"));
    input.pop(); // remove trailing newline
    println!("{}", count_laternfish(&input, 80));
    println!("{}", count_laternfish(&input, 256));
}

#[cfg(test)]
mod tests {
    use crate::count_laternfish;

    #[test]
    fn test_count_laternfish() {
        let data = "3,4,3,1,2";
        assert_eq!(26, count_laternfish(&data, 18));
        assert_eq!(5934, count_laternfish(&data, 80));
        assert_eq!(26984457539, count_laternfish(&data, 256));
    }
}
