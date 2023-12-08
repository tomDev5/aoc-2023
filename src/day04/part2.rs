use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &'static str = include_str!("../../data/day04/input.txt");

fn main() {
    let points = INPUT
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
        .collect_vec();
    let mut points = points.into_iter().map(|i| (1, i)).collect_vec();
    for i in 0..points.len() {
        for j in 0..points[i].1 {
            points[1 + i + j].0 += points[i].0;
        }
    }
    let result: usize = points.into_iter().map(|(r, _)| r).sum();
    println!("{:?}", result);
}
