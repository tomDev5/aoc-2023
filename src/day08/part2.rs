use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &'static str = include_str!("../../data/day08/input.txt");

fn main() {
    let mut lines = INPUT.split("\n").filter(|part| part.len() > 0);
    let actions = lines.next().unwrap_or_default().chars().collect_vec();

    let table = lines
        .filter_map(|line| {
            line.split([' ', '=', ')', '(', ','])
                .filter(|s| !s.is_empty())
                .collect_tuple::<(_, _, _)>()
        })
        .fold(HashMap::new(), |mut table, (from, left, right)| {
            table.insert(from.to_string(), (left.to_string(), right.to_string()));
            table
        });
    let cursors = table
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|start| find_iterations_to_z(&table, &actions, start))
        .fold(1, |acc, x| lcm(acc, x));
    println!("{:?}", cursors);
}

fn find_iterations_to_z(
    table: &HashMap<String, (String, String)>,
    actions: &[char],
    start: &str,
) -> usize {
    let mut cursor = start.to_string();
    let mut iteration = 0;
    while !cursor.ends_with("Z") {
        let action = actions[iteration % actions.len()];
        cursor = match action {
            'L' => table[&cursor].0.clone(),
            'R' => table[&cursor].1.clone(),
            _ => panic!("Invalid input"),
        };
        iteration += 1;
    }
    iteration
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
