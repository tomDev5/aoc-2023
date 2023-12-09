use crate::race::Race;
use itertools::Itertools;

mod race;

const INPUT: &'static str = include_str!("../../data/day06/input.txt");

fn main() {
    let race = INPUT
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .skip(1)
                .fold(String::new(), |acc, s| acc + s)
                .parse::<usize>()
                .ok()
        })
        .collect_vec();

    let result: usize = Race::new(race[0], race[1]).find_number_of_winning_runs();
    println!("{:?}", result);
}
