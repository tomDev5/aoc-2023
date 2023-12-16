use itertools::Itertools;
use transpose::Transpose;

use crate::mirror_finder::find_almost_mirror_line;

mod mirror_finder;
mod transpose;

const INPUT: &str = include_str!("../../data/day13/input.txt");

fn main() {
    let blocks = INPUT
        .split("\n\n")
        .map(|block| block.lines().map(|l| l.chars().collect_vec()).collect_vec())
        .collect_vec();

    let result: usize = blocks
        .into_iter()
        .filter_map(|block| {
            let mirror_line = find_almost_mirror_line(&block, 0);
            let mirror_column = find_almost_mirror_line(&block.transpose(), 0);

            match (mirror_line, mirror_column) {
                (None, None) => None,
                (None, Some(mirror_column)) => Some(mirror_column),
                (Some(mirror_line), None) => Some(100 * mirror_line),
                (Some(_), Some(_)) => None,
            }
        })
        .sum();

    println!("result: {:?}", result)
}
