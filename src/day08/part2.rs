use std::collections::HashMap;

use itertools::Itertools;

use crate::{action::Action, node::Node};

mod action;
mod node;

const INPUT: &'static str = include_str!("../../data/day08/input.txt");

fn main() {
    let mut lines = INPUT.split("\n").filter(|part| part.len() > 0);
    let actions = lines
        .next()
        .unwrap_or_default()
        .chars()
        .filter_map(|c| Action::try_from(c).ok())
        .collect_vec();

    let table = lines
        .filter_map(|line| {
            line.split([' ', '=', ')', '(', ','])
                .filter(|s| !s.is_empty())
                .map(|node| Node::from(node))
                .collect_tuple::<(_, _, _)>()
        })
        .fold(HashMap::new(), |mut table, (from, left, right)| {
            table.insert(from, (left, right));
            table
        });

    let number_of_iterations = table
        .keys()
        .filter(|k| k.is_part_start())
        .map(|start| find_iterations_to_z(&table, &actions, start, 0))
        .fold(1, lcm);
    println!("{:?}", number_of_iterations);
}

fn find_iterations_to_z(
    table: &HashMap<Node, (Node, Node)>,
    actions: &[Action],
    cursor: &Node,
    iteration: usize,
) -> usize {
    match cursor.is_part_end() {
        true => iteration,
        false => {
            let next_cursor = match actions[iteration % actions.len()] {
                Action::L => table.get(cursor).map(|t| &t.0),
                Action::R => table.get(cursor).map(|t| &t.1),
            };

            match next_cursor {
                Some(c) => find_iterations_to_z(table, actions, &c, iteration + 1),
                None => unreachable!("Invalid cursor in the table"),
            }
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
