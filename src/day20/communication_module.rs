use std::collections::HashMap;

use itertools::Itertools;

use crate::pulse::Pulse;

#[derive(Debug, Clone)]
pub enum CommunicationModule {
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
    Broadcast(Vec<String>),
}

impl CommunicationModule {
    pub fn from_input(input: &str) -> Option<(String, Self)> {
        let mut words = input.split_whitespace().map(|word| word.replace(',', ""));
        let name = words.next()?;
        words.next()?; // skip "->"
        Some(match name.chars().nth(0)? {
            '%' => (name[1..].into(), Self::FlipFlop(false, words.collect_vec())),
            '&' => (
                name[1..].into(),
                Self::Conjunction(HashMap::new(), words.collect_vec()),
            ),
            _ => (name, Self::Broadcast(words.collect_vec())),
        })
    }

    /// send a pulse the the module, get a list of modules and pulses to send to them
    pub fn send_pulse(&mut self, from: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        let mut send = Vec::new();
        match self {
            CommunicationModule::FlipFlop(on, destinations) => match pulse {
                Pulse::High => {}
                Pulse::Low => match on {
                    true => {
                        *on = false;
                        send.extend(destinations.iter().map(|d| (d.clone(), Pulse::Low)));
                    }
                    false => {
                        *on = true;
                        send.extend(destinations.iter().map(|d| (d.clone(), Pulse::High)));
                    }
                },
            },
            CommunicationModule::Conjunction(input_memory, destinations) => {
                input_memory.insert(from, pulse);

                if input_memory.values().all(|p| *p == Pulse::High) {
                    send.extend(destinations.iter_mut().map(|d| (d.clone(), Pulse::Low)))
                } else if input_memory.values().all(|p| *p == Pulse::Low) {
                    send.extend(destinations.iter().map(|d| (d.clone(), Pulse::High)))
                }
            }
            CommunicationModule::Broadcast(destinations) => {
                send.extend(destinations.iter().map(|d| (d.clone(), pulse)))
            }
        }
        send
    }
}
