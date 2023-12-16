use crate::hash::hash;

mod hash;

const INPUT: &str = include_str!("../../data/day15/input.txt");

fn main() {
    let result: usize = INPUT.split(',').map(|s| hash(s) as usize).sum();
    println!("result: {}", result);
}
