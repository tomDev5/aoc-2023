use itertools::Itertools;
use transpose::Transpose;

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
            let mirror_line = find_almost_mirror_line(&block);
            let mirror_column = find_almost_mirror_line(&block.transpose());

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

fn find_almost_mirror_line(matrix: &Vec<Vec<char>>) -> Option<usize> {
    for line_index in 1..matrix.len() {
        if verify_almost_mirror_line(matrix, line_index)? {
            return Some(line_index);
        }
    }
    None
}

fn verify_almost_mirror_line(matrix: &Vec<Vec<char>>, line: usize) -> Option<bool> {
    let mirrored_lines = usize::min(line, matrix.len() - line);

    let above_lines = matrix
        .get(line - mirrored_lines..line)?
        .into_iter()
        .collect_vec();
    let below_lines = matrix
        .get(line..line + mirrored_lines)?
        .into_iter()
        .rev()
        .collect_vec();
    let mut diff_counter = 0;
    for i in 0..mirrored_lines {
        let above_line = above_lines[i];
        let below_line = below_lines[i];
        for j in 0..above_line.len() {
            if above_line[j] != below_line[j] {
                diff_counter += 1;
            }
        }
    }

    Some(diff_counter == 1)
}
