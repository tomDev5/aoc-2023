use crate::derivative_predictor::get_prediction;

mod derivative_predictor;

const INPUT: &str = include_str!("../../data/day09/input.txt");

fn main() {
    let result: isize = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|number| number.parse::<isize>().ok())
        })
        .map(Iterator::rev)
        .filter_map(get_prediction)
        .sum();

    println!("{:?}", result);
}
