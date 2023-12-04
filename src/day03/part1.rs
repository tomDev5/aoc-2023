use crate::schematic::Schematic;

mod schematic;

const INPUT: &'static str = include_str!("../../data/day03/input.txt");

fn main() {
    let schematic = Schematic::new(INPUT);
    let result: usize = schematic.get_part_numbers().sum();
    println!("{:?}", result);
}
