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

pub struct MappingRange {
    from_start: usize,
    to_start: usize,
    length: usize,
}

impl MappingRange {
    pub fn new(from_start: usize, to_start: usize, length: usize) -> Self {
        Self {
            from_start,
            to_start,
            length,
        }
    }

    fn convert(&self, from: usize) -> Option<usize> {
        if (self.from_start..self.from_start + self.length).contains(&from) {
            Some((from - self.from_start) + self.to_start)
        } else {
            None
        }
    }
}
