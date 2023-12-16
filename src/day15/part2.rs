use std::collections::HashMap;

use itertools::Itertools;

use crate::hash::hash;

mod hash;

const INPUT: &str = include_str!("../../data/day15/input.txt");

enum Command {
    Insert(String, u8),
    Remove(String),
}

fn main() {
    let result: usize = INPUT
        .split(',')
        .filter_map(|s| {
            Some(if s.ends_with('-') {
                Command::Remove(s.split('-').next()?.to_string())
            } else {
                let mut split = s.split('=');
                Command::Insert(split.next()?.to_string(), split.next()?.parse::<u8>().ok()?)
            })
        })
        .enumerate()
        .fold(
            vec![HashMap::new(); 256],
            |mut maps, (command_index, command)| {
                match command {
                    Command::Insert(label, focal) => {
                        let map = &mut maps[hash(&label) as usize];
                        let i = map
                            .get(&label)
                            .map(|l: &(_, _)| l.0)
                            .unwrap_or(command_index);
                        map.insert(label, (i, focal));
                    }
                    Command::Remove(label) => {
                        let map = &mut maps[hash(&label) as usize];
                        map.remove(&label);
                    }
                }
                maps
            },
        )
        .into_iter()
        .enumerate()
        .map(|(box_index, map)| {
            map.into_iter()
                .sorted_by_key(|(_, (i, _))| *i)
                .map(|(_, (_, focal))| focal)
                .enumerate()
                .map(|(slot_number, focal)| (box_index, slot_number, focal))
                .collect_vec()
        })
        .flatten()
        .map(|(box_index, slot_number, focal)| (box_index + 1) * (slot_number + 1) * focal as usize)
        .sum();
    println!("result: {:?}", result);
}
