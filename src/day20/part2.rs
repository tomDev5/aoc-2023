use crate::circuit::Circuit;
use std::collections::HashMap;

mod circuit;
mod communication_module;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut circuit = Circuit::new(INPUT).expect("Invalid input");

    let final_module = circuit.find_modules_outputing_to("rx")[0];

    let semifinal_modules = circuit.find_modules_outputing_to(final_module);

    let mut semifinal_cycles: HashMap<String, Vec<usize>> = HashMap::from_iter(
        semifinal_modules
            .into_iter()
            .map(|m| (m.into(), Vec::new())),
    );

    let mut i = 0;
    while semifinal_cycles.values().any(|v| v.len() < 2) {
        i += 1;
        let pulse_counters = circuit.press_button();

        for (semifinal_module_name, hit_indexes) in &mut semifinal_cycles {
            if let Some((_, current_high)) = pulse_counters.get(semifinal_module_name) {
                if *current_high == 0 {
                    continue;
                }

                hit_indexes.push(i);
            }
        }
    }

    let result = semifinal_cycles
        .values()
        .cloned()
        .map(|hit_indexes| usize::abs_diff(hit_indexes[0], hit_indexes[1]))
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
