use itertools::Itertools;

use crate::joker_camel_cards::{Card, Hand};

const INPUT: &str = include_str!("../../data/day07/input.txt");

mod joker_camel_cards;

fn main() {
    let result: usize = INPUT
        .lines()
        .filter_map(|hand_and_bid| hand_and_bid.split_whitespace().collect_tuple())
        .filter_map(|(hand, bid)| {
            Some((
                Hand::new(hand.chars().filter_map(|c| Card::try_from(c).ok())),
                bid.parse::<usize>().ok()?,
            ))
        })
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();
    println!("{:?}", result);
}
