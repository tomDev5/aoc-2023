use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use condition::Condition;
use itertools::Itertools;

use crate::{action::Action, category::Category, workflow::Workflow};

mod action;
mod category;
mod condition;
mod part;
mod workflow;

const INPUT: &str = include_str!("../../data/day19/input.txt");

fn main() {
    let Some((workflows, _)) = INPUT.split("\n\n").collect_tuple() else {
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

    let mut total = 0;
    let paths = get_all_paths(&workflows, "in".to_string());
    for path in paths {
        let mut possible_ranges = HashMap::from([
            (Category::ExtremelyCoolLooking, (0..=4000usize)),
            (Category::Musical, (0..=4000usize)),
            (Category::Aerodynamic, (0..=4000usize)),
            (Category::Shiny, (0..=4000usize)),
        ]);
        for condition in &path {
            match condition.ordering {
                std::cmp::Ordering::Less => {
                    let range = possible_ranges
                        .get_mut(&condition.category)
                        .expect("Category not found");
                    *range = *range.start()..=usize::min(condition.rhs - 1, *range.end());
                }
                std::cmp::Ordering::Greater => {
                    let range = possible_ranges
                        .get_mut(&condition.category)
                        .expect("Category not found");
                    *range = usize::max(condition.rhs + 1, *range.start())..=*range.end();
                }
                std::cmp::Ordering::Equal => unreachable!(),
            };
        }
        total += possible_ranges
            .into_iter()
            .map(|(_, h)| h.end() + 1 - h.start())
            .product::<usize>();
    }

    println!("{:?}", total);
}

fn get_all_paths(workflows: &HashMap<String, Workflow>, workflow: String) -> Vec<Vec<Condition>> {
    let workflow = workflows.get(&workflow).expect("Workflow not found");
    let mut paths: Vec<Vec<Condition>> = Vec::new();
    for condition_index in 0..workflow.conditions.len() {
        match &workflow.conditions[condition_index].action {
            Action::Reject => continue,
            Action::Accept => {
                // we found a path that accepts
                // we need to add a path signaling this - with the condition and all the previous conditions as negatives
                let mut path = vec![workflow.conditions[condition_index].clone()];
                for previous_condition_index in 0..condition_index {
                    path.push(workflow.conditions[previous_condition_index].get_negative())
                }
                paths.push(path);
            }
            Action::SendTo(next_workflow) => {
                // we found a path that sends us to another workflow
                // we need to add paths signaling this - every path from the next workflow, with the condition and all the previous conditions as negatives
                let paths_from_next_workflow = get_all_paths(workflows, next_workflow.clone());
                let mut paths_from_next_workflow_with_current = vec![];
                for path_from_next_workflow in paths_from_next_workflow {
                    let mut path_from_next_workflow = path_from_next_workflow;
                    path_from_next_workflow.push(workflow.conditions[condition_index].clone());
                    for previous_condition_index in 0..condition_index {
                        path_from_next_workflow
                            .push(workflow.conditions[previous_condition_index].get_negative());
                    }
                    paths_from_next_workflow_with_current.push(path_from_next_workflow);
                }
                paths.extend(paths_from_next_workflow_with_current);
            }
        }
    }
    // default - if all conditions are not met
    match &workflow.default {
        Action::Accept => {
            let path = workflow
                .conditions
                .iter()
                .map(|c| c.get_negative())
                .collect_vec();
            paths.push(path);
        }
        Action::Reject => {}
        Action::SendTo(next_workflow) => {
            let current_condition_negatives = workflow
                .conditions
                .iter()
                .map(|c| c.get_negative())
                .collect_vec();
            let default_paths = get_all_paths(workflows, next_workflow.clone())
                .into_iter()
                .map(|mut path| {
                    path.extend(current_condition_negatives.clone());
                    path
                })
                .collect_vec();

            paths.extend(default_paths);
        }
    }
    paths
}
