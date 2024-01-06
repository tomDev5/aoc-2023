use std::collections::HashMap;

use itertools::Itertools;

use crate::{action::Action, part::Part, workflow::Workflow};

mod action;
mod category;
mod condition;
mod part;
mod workflow;

const INPUT: &str = include_str!("../../data/day19/input.txt");

fn main() {
    let Some((workflows, parts)) = INPUT.split("\n\n").collect_tuple() else {
        return;
    };

    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .filter_map(|line| {
            let line = &line[..line.len() - 1];
            let (name, line) = line.split('{').collect_tuple()?;
            Some((name.to_string(), Workflow::try_from(line).ok()?))
        })
        .collect();

    let accepted_parts: usize = parts
        .lines()
        .filter_map(|line| Part::try_from(line).ok())
        .filter_map(|part| {
            let mut current_workflow = "in".to_string();
            loop {
                let workflow = workflows.get(&current_workflow)?;
                match workflow.get_action(&part) {
                    Action::Accept => {
                        return Some(part);
                    }
                    Action::Reject => return None,
                    Action::SendTo(workflow) => current_workflow = workflow,
                }
            }
        })
        .map(|part| part.0.values().sum::<usize>())
        .sum();

    println!("{:?}", accepted_parts);
}
