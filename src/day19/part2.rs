use std::{cmp::Ordering, collections::HashMap, iter, ops::RangeInclusive};

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

    println!(
        "{:?}",
        get_possible_combination_count(&workflows, 1..=4000usize)
    );
}

fn get_possible_combination_count(
    workflows: &HashMap<String, Workflow>,
    default_range: RangeInclusive<usize>,
) -> usize {
    get_all_paths(workflows, "in".to_string())
        .into_iter()
        .map(|path| {
            let possible_ranges = path.iter().fold(
                HashMap::from([
                    (Category::ExtremelyCoolLooking, default_range.clone()),
                    (Category::Musical, default_range.clone()),
                    (Category::Aerodynamic, default_range.clone()),
                    (Category::Shiny, default_range.clone()),
                ]),
                |mut possible_ranges, condition| {
                    match condition.ordering {
                        Ordering::Less => {
                            let range = possible_ranges
                                .get_mut(&condition.category)
                                .expect("Category not found");
                            *range = *range.start()..=usize::min(condition.rhs - 1, *range.end());
                        }
                        Ordering::Greater => {
                            let range = possible_ranges
                                .get_mut(&condition.category)
                                .expect("Category not found");
                            *range = usize::max(condition.rhs + 1, *range.start())..=*range.end();
                        }
                        Ordering::Equal => unreachable!(),
                    };
                    possible_ranges
                },
            );

            possible_ranges
                .into_values()
                .map(|h| h.end() + 1 - h.start())
                .product::<usize>()
        })
        .sum()
}

/// Returns all possible paths from the given workflow
fn get_all_paths(workflows: &HashMap<String, Workflow>, workflow: String) -> Vec<Vec<Condition>> {
    let workflow = workflows.get(&workflow).expect("Workflow not found");
    let mut paths = Vec::new();

    // include workflow.default as a condition (which will always be True, as the minimal field value is 1)
    let conditions = workflow
        .conditions
        .clone()
        .into_iter()
        .chain(iter::once(Condition {
            ordering: Ordering::Less,
            category: Category::ExtremelyCoolLooking,
            rhs: usize::MAX,
            action: workflow.default.clone(),
        }))
        .collect_vec();

    // iterate all conditions
    for condition_index in 0..conditions.len() {
        // the state that allows a condition to be accepted - the condition itself is True, and all the previous conditions are False
        let current_conditions = conditions[0..condition_index]
            .iter()
            .map(Condition::get_negative)
            .chain(iter::once(conditions[condition_index].clone()));

        match &conditions[condition_index].action {
            Action::Reject => continue,
            Action::Accept => {
                paths.push(current_conditions.collect_vec());
            }
            Action::SendTo(next_workflow) => {
                let paths_from_next_workflow_with_current =
                    get_all_paths(workflows, next_workflow.clone())
                        .into_iter()
                        .map(|mut path| {
                            path.extend(current_conditions.clone());
                            path
                        })
                        .collect_vec();

                paths.extend(paths_from_next_workflow_with_current);
            }
        }
    }
    paths
}
