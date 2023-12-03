use itertools::Itertools;

const INPUT: &'static str = include_str!("../../data/day03/input.txt");

mod schematic;

fn main() {
    let sum: usize = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c.is_digit(10))
                .fold(Vec::<(usize, String)>::new(), |mut v, (index, digit)| {
                    if let Some((last_index, number)) = v
                        .last_mut()
                        .filter(|(last_index, _)| *last_index == index - 1)
                    {
                        *last_index = index;
                        number.push(digit);
                    } else {
                        v.push((index, digit.to_string()));
                    }
                    v
                })
        })
        .enumerate()
        .map(|(line_number, end_indexes_and_numbers)| {
            end_indexes_and_numbers
                .into_iter()
                .map(|(end_index, number)| (line_number, end_index + 1 - number.len(), number))
                .collect_vec()
        })
        .flatten()
        .filter(|(number_line, number_start_column, number)| {
            // block range
            let start_line = number_line.checked_sub(1).unwrap_or(0);
            let end_line = number_line + 1;
            let start_col = number_start_column.checked_sub(1).unwrap_or(0);
            let end_col = number_start_column + number.len();

            // get number of special chars in the block
            let special_chars_in_block = INPUT
                .lines()
                .into_iter()
                .skip(start_line)
                .take(end_line - start_line + 1)
                .map(|line| {
                    line.chars()
                        .into_iter()
                        .skip(start_col)
                        .take(end_col - start_col + 1)
                })
                .flatten()
                .filter(|c| !c.is_digit(10) && *c != '.')
                .count();

            special_chars_in_block > 0
        })
        .filter_map(|(_, _, number)| number.parse::<usize>().ok())
        .sum();
    println!("{:?}", sum);
    // find numbers, for each number find start and end column
    // from that gather the block
    // search block for symbols
}
