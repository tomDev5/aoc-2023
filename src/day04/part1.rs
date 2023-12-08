use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &'static str = include_str!("../../data/day04/input.txt");

fn main() {
    let result = INPUT
        .lines()
        .filter_map(|line| line.split('|').collect_tuple())
        .map(|(winning, got): (&str, &str)| (winning.split_whitespace(), got.split_whitespace()))
        .map(|(winning, got)| {
            (
                winning.filter_map(|n| n.parse::<usize>().ok()),
                got.filter_map(|n| n.parse::<usize>().ok()),
            )
        })
        .map(|(winning, got)| {
            (
                HashSet::<usize>::from_iter(winning),
                HashSet::<usize>::from_iter(got),
            )
        })
        .map(|(winning, got)| winning.intersection(&got).count())
        .filter(|winning_cards| *winning_cards > 0)
        .fold(0, |acc, winning_cards| {
            let points = 2usize.pow(winning_cards as u32 - 1);
            acc + points
        });
    println!("{:?}", result);
}
