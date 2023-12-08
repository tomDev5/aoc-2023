use std::usize;

use itertools::Itertools;

const INPUT: &'static str = include_str!("../../data/day05/input.txt");

struct RangeMapping {
    from_start: usize,
    to_start: usize,
    length: usize,
}

impl RangeMapping {
    fn new(from_start: usize, to_start: usize, length: usize) -> Self {
        Self {
            from_start,
            to_start,
            length,
        }
    }

    fn convert(&self, from: usize) -> Option<usize> {
        if from >= self.from_start && from < self.from_start + self.length {
            Some((from - self.from_start) + self.to_start)
        } else {
            None
        }
    }
}

struct MappingBlock {
    ranges: Vec<RangeMapping>,
}

impl MappingBlock {
    fn new(ranges: impl Iterator<Item = RangeMapping>) -> Self {
        Self {
            ranges: ranges.collect_vec(),
        }
    }

    fn convert(&self, from: usize) -> Option<usize> {
        for range_mapping in &self.ranges {
            if let Some(new_index) = range_mapping.convert(from) {
                return Some(new_index);
            }
        }
        None
    }
}

fn main() {
    let maps: Vec<MappingBlock> = INPUT
        .split("\n\n")
        .skip(1)
        .map(|block| {
            let mut lines = block.lines();
            let _title = lines.next();
            let rules = MappingBlock::new(lines.map(|rule| {
                let tuple = rule
                    .split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_vec();
                RangeMapping::new(tuple[1], tuple[0], tuple[2])
            }));
            rules
        })
        .collect_vec();

    let result = INPUT
        .lines()
        .nth(0)
        .expect("Invalid input")
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|seed| {
            let mut index = seed;
            for map in &maps {
                index = map.convert(index).unwrap_or(index);
            }
            index
        })
        .min();
    println!("{:?}", result);
}
