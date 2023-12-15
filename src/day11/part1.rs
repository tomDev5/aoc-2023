use cosmic_element::CosmicElement;
use itertools::Itertools;
use transpose::Transpose;

mod cosmic_element;
mod transpose;

const INPUT: &'static str = include_str!("../../data/day11/input.txt");

fn main() {
    let matrix = INPUT
        .lines()
        .map(|line| line.chars().filter_map(|c| CosmicElement::try_from(c).ok()));
    let matrix = matrix.map(|l| l.collect_vec()).collect_vec();
    let empty_lines = matrix
        .iter()
        .enumerate()
        .filter(|(_, l)| l.iter().all(|x| *x == CosmicElement::Void))
        .map(|x| x.0)
        .collect_vec();
    let empty_cols = matrix
        .clone()
        .transpose()
        .iter()
        .enumerate()
        .filter(|(_, l)| l.iter().all(|x| *x == CosmicElement::Void))
        .map(|x| x.0)
        .collect_vec();
    let galaxies = matrix
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(column_index, element)| {
                    if *element == CosmicElement::Galaxy {
                        Some((line_index, column_index))
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
        .map(|(p0, p1)| {
            // add 1 for every empty line in the range
            let empty_lines_crossed = empty_lines
                .iter()
                .filter(|l| l >= &&usize::min(p0.0, p1.0) && l <= &&usize::max(p0.0, p1.0))
                .count();
            let empty_cols_crossed = empty_cols
                .iter()
                .filter(|l| l >= &&usize::min(p0.1, p1.1) && l <= &&usize::max(p0.1, p1.1))
                .count();

            distance(p0, p1, empty_lines_crossed + empty_cols_crossed)
        })
        .sum::<usize>()
        / 2;

    println!("result: {:?}", distance_sum);
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

fn distance(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    empty_rows_or_cols: usize,
) -> usize {
    usize::abs_diff(x1, x2) + usize::abs_diff(y1, y2) + empty_rows_or_cols
}
