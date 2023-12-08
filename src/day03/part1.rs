use crate::schematic::Schematic;

mod schematic;

const INPUT: &'static str = include_str!("../../data/day03/input.txt");

fn main() {
    let schematic = Schematic::new(INPUT);
    let result: usize = schematic.all_parts().sum();
    println!("{:?}", result);
}
