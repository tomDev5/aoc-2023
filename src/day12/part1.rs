use itertools::Itertools;

const INPUT: &str = include_str!("../../data/day12/input.txt");

fn main() {
    let result: usize = INPUT
        .lines()
        .filter_map(|line| line.split_whitespace().collect_tuple())
        .map(|(line, hash_groups)| {
            (
                line,
                hash_groups
                    .split(',')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect_vec(),
            )
        })
        .map(|(line, hash_groups)| get_valid_count(line, &hash_groups))
        .sum();

    println!("result: {:?}", result)
}

fn get_valid_count(line: &str, hash_groups: &[usize]) -> usize {
    let Some(q_index) = line.find('?') else {
        match is_valid(line, hash_groups) {
            true => return 1,
            false => return 0,
        }
    };

    let mut line = line.to_string();
    line.replace_range(q_index..q_index + 1, "#");
    let a = get_valid_count(&line, hash_groups);
    line.replace_range(q_index..q_index + 1, ".");
    let b = get_valid_count(&line, hash_groups);
    a + b
}

fn is_valid(line: &str, hash_groups: &[usize]) -> bool {
    line.split('.')
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .eq(hash_groups.iter().cloned())
}
