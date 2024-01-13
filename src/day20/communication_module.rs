use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pulse {
    High,
    Low,
}

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
            '%' => (name[1..].into(), Self::FlipFlop(false, words.collect())),
            '&' => (
                name[1..].into(),
                Self::Conjunction(HashMap::new(), words.collect()),
            ),
            _ => (name, Self::Broadcast(words.collect())),
        })
    }

    pub fn get_destination(&self) -> &Vec<String> {
        match self {
            CommunicationModule::FlipFlop(_, destinations) => destinations,
            CommunicationModule::Conjunction(_, destinations) => destinations,
            CommunicationModule::Broadcast(destinations) => destinations,
        }
    }

    /// send a pulse the the module, get a list of modules and pulses to send to them
    pub fn send_pulse(&mut self, from: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        let mut send = Vec::new();

        match self {
            CommunicationModule::FlipFlop(on, destinations) => {
                if let Pulse::Low = pulse {
                    *on = !*on;
                    let pulse_value = if *on { Pulse::High } else { Pulse::Low };
                    send.extend(destinations.iter().map(|d| (d.clone(), pulse_value)));
                }
            }
            CommunicationModule::Conjunction(input_memory, destinations) => {
                input_memory.insert(from, pulse);
                let pulse_value = if input_memory.values().all(|p| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                send.extend(destinations.iter().map(|d| (d.clone(), pulse_value)));
            }
            CommunicationModule::Broadcast(destinations) => {
                send.extend(destinations.iter().map(|d| (d.clone(), pulse)));
            }
        }

        send
    }
}
