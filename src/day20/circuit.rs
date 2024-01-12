use std::collections::HashMap;

use itertools::Itertools;

use crate::{communication_module::CommunicationModule, pulse::Pulse};

pub struct Circuit {
    modules: HashMap<String, CommunicationModule>,
}

impl Circuit {
    pub fn new(input: &str) -> Option<Self> {
        let mut modules: HashMap<String, CommunicationModule> = input
            .lines()
            .map(CommunicationModule::from_input)
            .collect::<Option<_>>()?;

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
                    module
                        .get_destination()
                        .iter()
                        .any(|d| d == &conjunction_module)
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
        Some(Self { modules })
    }

    pub fn press_button(&mut self) -> (usize, usize) {
        let mut pulses_to_send =
            vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
        let mut total_low = 1;
        let mut total_high = 0;

        while let Some((from, destination, pulse)) = pulses_to_send.pop() {
            if let Some(module) = self.modules.get_mut(&destination) {
                let next = module.send_pulse(from.clone(), pulse);
                total_low += next.iter().filter(|(_, p)| *p == Pulse::Low).count();
                total_high += next.iter().filter(|(_, p)| *p == Pulse::High).count();
                pulses_to_send.extend(
                    next.into_iter().map(|(next_dest, next_pulse)| {
                        (destination.clone(), next_dest, next_pulse)
                    }),
                );
            }
        }

        (total_low, total_high)
    }
}
