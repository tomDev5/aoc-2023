use crate::circuit::Circuit;
use std::collections::HashMap;

mod circuit;
mod communication_module;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut circuit = Circuit::new(INPUT).expect("Invalid input");

    let mut semifinal_cycles: HashMap<String, (usize, usize)> = HashMap::from_iter([
        ("sg".to_string(), (0, 0)),
        ("lm".to_string(), (0, 0)),
        ("dh".to_string(), (0, 0)),
        ("db".to_string(), (0, 0)),
    ]);

    let mut i = 0;
    while semifinal_cycles.values().any(|(_, b)| *b == 0) {
        i += 1;
        let pulse_counters = circuit.press_button();

        for (semifinal_module_name, (first_hit_iteration, second_hit_iteration)) in
            &mut semifinal_cycles
        {
            if let Some((_, current_high)) = pulse_counters.get(semifinal_module_name) {
                if *current_high == 0 {
                    continue;
                }

                if *first_hit_iteration == 0 {
                    *first_hit_iteration = i;
                } else if *second_hit_iteration == 0 {
                    *second_hit_iteration = i;
                }
            }
        }
    }

    let result = semifinal_cycles
        .values()
        .cloned()
        .map(|(a, b)| usize::abs_diff(a, b))
        .fold(1, lcm);

    println!("result: {:?}", result);
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
