use crate::mapping_block::{MappingBlock, MappingRange};
use itertools::Itertools;
use std::usize;

mod mapping_block;

const INPUT: &'static str = include_str!("../../data/day05/input.txt");

fn main() {
    let mapping_blocks: Vec<MappingBlock> = INPUT
        .split("\n\n")
        .skip(1)
        .map(|block| {
            let mut lines = block.lines();
            let _title = lines.next();
            MappingBlock::new(lines.map(|rule| {
                let tuple = rule
                    .split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_vec();
                MappingRange::new(tuple[1], tuple[0], tuple[2])
            }))
        })
        .collect_vec();

    let result = INPUT
        .lines()
        .nth(0)
        .expect("Invalid input")
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .chunks(2)
        .into_iter()
        .filter_map(|chunk| chunk.collect_tuple::<(_, _)>())
        .map(|(seed_start, seed_length)| (seed_start..seed_start + seed_length))
        .map(|seed_range| {
            seed_range.map(|seed| {
                mapping_blocks.iter().fold(seed, |index, mapping_block| {
                    mapping_block.convert(index).unwrap_or(index)
                })
            })
        })
        .flatten()
        .min();

    println!("{:?}", result);
}
