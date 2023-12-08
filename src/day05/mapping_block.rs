use std::ops::Range;

use itertools::Itertools;

pub struct MappingBlock {
    ranges: Vec<MappingRange>,
}

impl MappingBlock {
    pub fn new(ranges: impl Iterator<Item = MappingRange>) -> Self {
        Self {
            ranges: ranges.collect_vec(),
        }
    }

    pub fn convert(&self, from: usize) -> Option<usize> {
        for range_mapping in &self.ranges {
            if let Some(new_index) = range_mapping.convert(from) {
                return Some(new_index);
            }
        }
        None
    }
}

pub struct MappingRange(Range<usize>, usize);

impl MappingRange {
    pub fn new(from_start: usize, to_start: usize, length: usize) -> Self {
        Self(from_start..from_start + length, to_start)
    }

    fn convert(&self, from: usize) -> Option<usize> {
        if self.0.contains(&from) {
            Some((from - self.0.start) + self.1)
        } else {
            None
        }
    }
}
