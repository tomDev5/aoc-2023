use cached::proc_macro::cached;
use itertools::Itertools;
// use itertools::Itertools;

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
        .map(|(line, hash_groups)| num_valid_solutions(line.to_string(), hash_groups))
        .sum();

    println!("result: {:?}", result)
}

#[cached]
fn num_valid_solutions(record: String, groups: Vec<usize>) -> usize {
    if record.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }

    if groups.is_empty() {
        return if record.contains('#') { 0 } else { 1 };
    }

    let (c, rest_of_record) = record.split_at(1);
    match c {
        "." => num_valid_solutions(rest_of_record.to_string(), groups),
        "#" => {
            let group = groups[0];
            if record.len() >= group
                && record.chars().take(group).all(|c| c != '.')
                && (record.len() == group || record.get(group..group + 1) != Some("#"))
            {
                if record.len() > group {
                    num_valid_solutions(
                        record.split_at(group + 1).1.to_string(),
                        groups[1..].to_vec(),
                    )
                } else {
                    num_valid_solutions(String::default(), groups[1..].to_vec())
                }
            } else {
                0
            }
        }
        "?" => {
            num_valid_solutions(String::from("#") + rest_of_record, groups.clone())
                + num_valid_solutions(String::from(".") + rest_of_record, groups)
        }
        _ => unreachable!(),
    }
}
