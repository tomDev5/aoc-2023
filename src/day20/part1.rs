use itertools::Itertools;

use crate::{communication_module::CommunicationModule, pulse::Pulse};
use std::collections::HashMap;

mod communication_module;
mod pulse;

const INPUT: &str = include_str!("../../data/day20/input.txt");

fn main() {
    let mut modules: HashMap<String, CommunicationModule> = INPUT
        .lines()
        .filter_map(CommunicationModule::from_input)
        .collect();

    let conjunction_module_names = modules
        .iter()
        .filter_map(|(name, module)| {
            if let CommunicationModule::Conjunction(_, _) = module {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    for conjunction_module in conjunction_module_names {
        let modules_sending_to_conjunction_module = modules
            .iter()
            .filter(|(_, module)| {
                let destinations = match module {
                    CommunicationModule::FlipFlop(_, destinations) => destinations,
                    CommunicationModule::Conjunction(_, destinations) => destinations,
                    CommunicationModule::Broadcast(destinations) => destinations,
                };
                destinations.iter().any(|d| d == &conjunction_module)
            })
            .map(|(n, _)| n.to_string())
            .collect_vec();
        if let Some(CommunicationModule::Conjunction(inputs, _)) =
            modules.get_mut(&conjunction_module)
        {
            modules_sending_to_conjunction_module
                .into_iter()
                .for_each(|input| {
                    inputs.insert(input, Pulse::Low);
                })
        }
    }

    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        let (new_low, new_high) = press_button(&mut modules);
        total_low += new_low;
        total_high += new_high;
    }
    println!("{:?}, {:?}", total_low, total_high);
    println!("{:?}", total_low * total_high);
}

/// presses the button module, starting the chain.
/// returns the number of pulses sent
fn press_button(modules: &mut HashMap<String, CommunicationModule>) -> (usize, usize) {
    let mut pulses_to_send = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];

    let mut total_low = 1;
    let mut total_high = 0;
    while let Some((from, destination, pulse)) = pulses_to_send.pop() {
        let Some(module) = modules.get_mut(&destination) else {
            continue;
        };
        let next = module.send_pulse(from.to_string(), pulse);
        total_low += next.iter().filter(|(_, p)| *p == Pulse::Low).count();
        total_high += next.iter().filter(|(_, p)| *p == Pulse::High).count();
        pulses_to_send.extend(next.into_iter().map(|(next_destination, next_pulse)| {
            (destination.clone(), next_destination, next_pulse)
        }));
    }

    (total_low, total_high)
}
