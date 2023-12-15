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
    let galaxies = matrix
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(x, element)| {
                    if element == CosmicElement::Galaxy {
                        Some((x, y))
                    } else {
                        None
                    }
                })
        })
        .flatten();
    let distance_sum: usize = galaxies
        .clone()
        .cartesian_product(galaxies)
        .into_iter()
        .map(|(p0, p1)| distance(p0, p1))
        .sum::<usize>()
        / 2;
    println!("Sum of distances: {}", distance_sum);
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

fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    usize::abs_diff(x1, x2) + usize::abs_diff(y1, y2)
}
