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

    fn convert(&self, from: usize) -> usize {
        if from >= self.from_start && from < self.from_start + self.length {
            (from - self.from_start) + self.to_start
        } else {
            from
        }
    }
}

fn main() {
    let maps: Vec<Vec<RangeMapping>> = INPUT
        .split("\n\n")
        .skip(1)
        .map(|block| {
            let mut lines = block.lines();
            let _title = lines.next();
            let rules = lines
                .map(|rule| {
                    let tuple = rule
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect_vec();
                    RangeMapping::new(tuple[1], tuple[0], tuple[2])
                })
                .collect_vec();
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
                for range_mapping in map {
                    let new_index = range_mapping.convert(index);
                    if new_index != index {
                        index = new_index;
                        break;
                    }
                }
            }
            index
        })
        .min();
    println!("{:?}", result);
}
