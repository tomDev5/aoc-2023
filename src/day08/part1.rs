use std::collections::HashMap;

use itertools::Itertools;

use crate::{action::Action, node::Node};

mod action;
mod node;

const INPUT: &str = include_str!("../../data/day08/input.txt");

fn main() {
    let mut lines = INPUT.split('\n').filter(|part| !part.is_empty());
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
                .map(Node::from)
                .collect_tuple::<(_, _, _)>()
        })
        .fold(HashMap::new(), |mut table, (from, left, right)| {
            table.insert(from, (left, right));
            table
        });
    let iterations = table
        .keys().find(|k| k.is_full_start())
        .map(|start| find_iterations_to_z(&table, &actions, start, 0))
        .expect("Invalid input - missing full start");

    println!("{:?}", iterations);
}

fn find_iterations_to_z(
    table: &HashMap<Node, (Node, Node)>,
    actions: &[Action],
    cursor: &Node,
    iteration: usize,
) -> usize {
    match cursor.is_full_end() {
        true => iteration,
        false => {
            let next_cursor = match actions[iteration % actions.len()] {
                Action::L => table.get(cursor).map(|t| &t.0),
                Action::R => table.get(cursor).map(|t| &t.1),
            };

            match next_cursor {
                Some(c) => find_iterations_to_z(table, actions, c, iteration + 1),
                None => unreachable!("Invalid cursor in the table"),
            }
        }
    }
}
