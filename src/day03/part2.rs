use crate::schematic::Schematic;

mod schematic;

const INPUT: &str = include_str!("../../data/day03/input.txt");

fn main() {
    let schematic = Schematic::new(INPUT);
    let result: usize = schematic.gear_ratios().sum();
    println!("{:?}", result);
}
