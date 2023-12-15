use itertools::Itertools;
use std::fmt::Debug;
use transpose::Transpose;

mod transpose;

const INPUT: &'static str = include_str!("../../data/day11/input.txt");

#[derive(PartialEq, Clone, Copy)]
enum CosmicElement {
    Void,
    Galaxy,
}

impl Debug for CosmicElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for CosmicElement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(CosmicElement::Void),
            '#' => Ok(CosmicElement::Galaxy),
            _ => Err(()),
        }
    }
}

fn main() {
    let matrix = INPUT
        .lines()
        .map(|line| line.chars().filter_map(|c| CosmicElement::try_from(c).ok()));
    let matrix = matrix.map(|l| l.collect_vec()).collect_vec();
    let matrix = expand_universe(matrix);
    for l in matrix {
        for c in l {
            print!("{:?}", c);
        }
        println!();
    }
}

fn expand_universe(matrix: Vec<Vec<CosmicElement>>) -> Vec<Vec<CosmicElement>> {
    let mut expanded = Vec::new();
    for line in &matrix {
        expanded.push(line.clone());
        if line.iter().all(|c| *c == CosmicElement::Void) {
            expanded.push(line.clone());
        }
    }

    let matrix = expanded.transpose();
    let mut expanded = Vec::new();

    for column in &matrix {
        expanded.push(column.clone());
        if column.iter().all(|c| *c == CosmicElement::Void) {
            expanded.push(column.clone());
        }
    }
    expanded.transpose()
}
