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
        .map(|seed| {
            mapping_blocks.iter().fold(seed, |index, mapping_block| {
                mapping_block.convert(index).unwrap_or(index)
            })
        })
        .min();
    println!("{:?}", result);
}
