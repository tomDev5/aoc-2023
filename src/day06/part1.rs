use crate::race::Race;
use itertools::Itertools;
use transpose::Transpose;

mod race;
mod transpose;

const INPUT: &'static str = include_str!("../../data/day06/input.txt");

fn main() {
    let races = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec()
        })
        .collect_vec()
        .transpose()
        .into_iter()
        .map(|v| Race::new(v[0], v[1]));

    let result: usize = races
        .map(|race| race.find_number_of_winning_runs())
        .product();
    println!("{:?}", result);
}
