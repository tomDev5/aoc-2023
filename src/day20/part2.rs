use std::collections::HashMap;

use itertools::Itertools;

use crate::circuit::Circuit;

mod circuit;
mod communication_module;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut circuit = Circuit::new(INPUT).expect("Invalid input");

    let mut semifinal_cycles: HashMap<String, (usize, usize)> = HashMap::from_iter([
        (String::from("sg"), (0, 0)),
        (String::from("lm"), (0, 0)),
        (String::from("dh"), (0, 0)),
        (String::from("db"), (0, 0)),
    ]);
    let mut i = 0;
    loop {
        i += 1;
        let pulse_counters = circuit.press_button();
        for (name, (a, b)) in &mut semifinal_cycles {
            let Some((_, current_high)) = pulse_counters.get(name) else {
                continue;
            };
            if *current_high == 0 {
                continue;
            }

            if *a == 0 {
                *a = i;
            } else if *b == 0 {
                *b = i;
            }
        }

        if semifinal_cycles.values().all(|(_, b)| *b != 0) {
            break;
        }
    }

    println!("semifinal cycles: {:?}", semifinal_cycles);
    println!(
        "semifinal cycles: {:?}",
        semifinal_cycles.values().map(|(a, b)| b - a).fold(1, lcm)
    );
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
