use itertools::Itertools;

pub struct Schematic {
    matrix: Vec<Vec<char>>,
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        Self {
            matrix: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn get_part_numbers(&self) -> impl Iterator<Item = usize> {
        
        std::iter::empty()
    }
}
