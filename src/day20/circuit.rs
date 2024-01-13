use std::collections::HashMap;

use itertools::Itertools;

use crate::communication_module::{CommunicationModule, Pulse};

pub struct Circuit {
    pub modules: HashMap<String, CommunicationModule>,
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

    pub fn press_button(&mut self) -> HashMap<String, (usize, usize)> {
        let mut pulses_to_send =
            vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
        let mut counter: HashMap<String, (usize, usize)> = HashMap::new();

        while let Some((source, destination, pulse_type)) = pulses_to_send.pop() {
            let counter_entry = counter.entry(source.clone()).or_insert((0, 0));
            match pulse_type {
                Pulse::Low => counter_entry.0 += 1,
                Pulse::High => counter_entry.1 += 1,
            }
            if let Some(module) = self.modules.get_mut(&destination) {
                let next = module.send_pulse(source.clone(), pulse_type);
                pulses_to_send.splice(
                    ..0,
                    next.into_iter().map(|(next_destination, next_pulse)| {
                        (destination.clone(), next_destination, next_pulse)
                    }),
                );
            }
        }

        counter
    }

    #[allow(dead_code)]
    pub fn find_modules_outputing_to(&self, to: &str) -> Vec<&String> {
        self.modules
            .iter()
            .filter(|(_, module)| matches!(module, CommunicationModule::Conjunction(_, _)))
            .filter(|(_, module)| module.get_destination().contains(&to.to_string()))
            .map(|(name, _)| name)
            .collect_vec()
    }
}
