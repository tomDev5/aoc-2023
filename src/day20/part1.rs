use crate::circuit::Circuit;

mod circuit;
mod communication_module;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut circuit = Circuit::new(INPUT).expect("Invalid input");

    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        let pulse_counters = circuit.press_button();
        total_low += pulse_counters.values().map(|(low, _)| low).sum::<usize>();
        total_high += pulse_counters.values().map(|(_, high)| high).sum::<usize>();
    }

    println!("result: {:?}", total_low * total_high);
}
