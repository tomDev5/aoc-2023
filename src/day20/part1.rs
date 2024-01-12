use crate::circuit::Circuit;

mod circuit;
mod communication_module;
mod pulse;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut circuit = Circuit::new(INPUT).expect("Invalid input");

    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        let (new_low, new_high) = circuit.press_button();
        total_low += new_low;
        total_high += new_high;
    }

    println!("result: {:?}", total_low * total_high);
}
