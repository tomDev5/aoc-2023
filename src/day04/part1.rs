use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../data/day04/input.txt");

fn main() {
    let result = INPUT
        .lines()
        .filter_map(|line| line.split('|').collect_tuple())
        .map(|(winning, got): (&str, &str)| {
            (
                HashSet::<usize>::from_iter(
                    winning
                        .split_whitespace()
                        .filter_map(|n| n.parse::<usize>().ok()),
                ),
                HashSet::<usize>::from_iter(
                    got.split_whitespace()
                        .filter_map(|n| n.parse::<usize>().ok()),
                ),
            )
        })
        .map(|(winning, got)| winning.intersection(&got).count())
        .filter(|winning_cards| *winning_cards > 0)
        .fold(0, |points, winning_cards| {
            points + 2usize.pow(winning_cards as u32 - 1)
        });
    println!("{:?}", result);
}
