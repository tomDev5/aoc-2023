use itertools::Itertools;

const INPUT: &str = include_str!("../../data/day14/input.txt");

fn main() {
    let mut matrix = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    roll_north(&mut matrix);

    println!("result: {:?}", calculate_load(&matrix));
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

fn calculate_load(matrix: &[Vec<char>]) -> usize {
    matrix
        .iter()
        .rev()
        .enumerate()
        .map(|(index, line)| line.iter().filter(|c| **c == 'O').count() * (index + 1))
        .sum()
}
