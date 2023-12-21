use crate::transpose::Transpose;
use itertools::Itertools;

mod transpose;

const INPUT: &str = include_str!("../../data/day14/input.txt");

fn main() {
    let mut matrix = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    for _ in 0..1_000 {
        roll_north(&mut matrix);
        roll_west(&mut matrix);
        roll_south(&mut matrix);
        roll_east(&mut matrix);
    }

    println!("{}", calculate_load(&matrix));
}

fn roll_north(matrix: &mut Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'O' {
                let mut northmost_line = i;
                for line in (0..i).rev() {
                    if matrix[line][j] != '.' {
                        break;
                    }
                    northmost_line = line
                }
                matrix[northmost_line][j] = 'O';
                if i != northmost_line {
                    matrix[i][j] = '.';
                }
            }
        }
    }
}

fn roll_south(matrix: &mut Vec<Vec<char>>) {
    let mut reversed = matrix.iter().rev().map(|line| line.to_vec()).collect_vec();
    roll_north(&mut reversed);
    let rolled = reversed
        .iter()
        .rev()
        .map(|line| line.to_vec())
        .collect_vec();
    matrix.clear();
    matrix.extend(rolled);
}

fn roll_west(matrix: &mut Vec<Vec<char>>) {
    let mut side = matrix.clone().transpose();
    roll_north(&mut side);
    let rolled = side.transpose();
    matrix.clear();
    matrix.extend(rolled);
}

fn roll_east(matrix: &mut Vec<Vec<char>>) {
    let mut side = matrix.clone().transpose();
    roll_south(&mut side);
    let rolled = side.transpose();
    matrix.clear();
    matrix.extend(rolled);
}

fn calculate_load(matrix: &[Vec<char>]) -> usize {
    matrix
        .iter()
        .rev()
        .enumerate()
        .map(|(index, line)| line.iter().filter(|c| **c == 'O').count() * (index + 1))
        .sum()
}
