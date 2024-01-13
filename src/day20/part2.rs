use crate::circuit::Circuit;
use std::collections::HashMap;

mod circuit;
mod communication_module;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut circuit = Circuit::new(INPUT).expect("Invalid input");

    let mut semifinal_cycles: HashMap<String, (Option<usize>, Option<usize>)> =
        HashMap::from_iter([
            ("sg".to_string(), (None, None)),
            ("lm".to_string(), (None, None)),
            ("dh".to_string(), (None, None)),
            ("db".to_string(), (None, None)),
        ]);

    let mut i = 0;
    while semifinal_cycles
        .values()
        .any(|(a, b)| a.is_none() || b.is_none())
    {
        i += 1;
        let pulse_counters = circuit.press_button();

        for (semifinal_module_name, (first_hit_iteration, second_hit_iteration)) in
            &mut semifinal_cycles
        {
            if let Some((_, current_high)) = pulse_counters.get(semifinal_module_name) {
                if *current_high == 0 {
                    continue;
                }

                if first_hit_iteration.is_none() {
                    *first_hit_iteration = Some(i);
                } else if second_hit_iteration.is_none() {
                    *second_hit_iteration = Some(i);
                }
            }
        }
    }

    let result = semifinal_cycles
        .values()
        .cloned()
        .filter_map(|(a, b)| Some((a?, b?)))
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
