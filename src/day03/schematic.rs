use std::collections::HashMap;

use itertools::Itertools;

type Coordinates = (usize, usize);

pub struct Schematic {
    matrix: Vec<Vec<char>>,
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        Self {
            matrix: input.lines().map(|s| s.chars().collect_vec()).collect_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn all_parts(&self) -> impl Iterator<Item = usize> + '_ {
        self.numbers_and_special_chars()
            .filter_map(|(line, col, number, mut special_chars)| {
                match special_chars.next().is_some() {
                    true => Some((line, col, number)),
                    false => None,
                }
            })
            .map(|(_, _, number)| number)
    }

    #[allow(dead_code)]
    pub fn gear_ratios(&self) -> impl Iterator<Item = usize> + '_ {
        self.numbers_and_special_chars()
            .filter_map(|(_, _, number, special_chars)| {
                let mut stars = special_chars
                    .filter(|(_, _, c)| c.eq(&'*'))
                    .map(|(line, col, _)| (line, col))
                    .peekable();
                match stars.peek().is_some() {
                    true => Some((number, stars)),
                    false => None,
                }
            })
            .fold(
                HashMap::<Coordinates, Vec<usize>>::new(),
                |mut map, (number, stars)| {
                    stars.for_each(|star_coordinates| {
                        map.entry(star_coordinates).or_default().push(number);
                    });
                    map
                },
            )
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn numbers_and_special_chars(
        &self,
    ) -> impl Iterator<
        Item = (
            usize,
            usize,
            usize,
            impl Iterator<Item = (usize, usize, char)> + '_,
        ),
    > + '_ {
        self.get_numbers_in_matrix()
            .filter_map(|(line, col)| {
                self.get_number_at_index(line, col)
                    .map(|number| (line, col, number))
            })
            .map(|(line, col, number)| {
                let start_line = line.saturating_sub(1);
                let end_line = line + 1;
                let start_col = col.saturating_sub(1);
                let end_col = col + number.len();

                let special_chars_in_block =
                    self.get_chars_in_block(start_line, end_line, start_col, end_col);
                (line, col, number, special_chars_in_block)
            })
            .filter_map(|(line, col, number, special_chars_in_block)| {
                number
                    .parse::<usize>()
                    .ok()
                    .map(|number| (line, col, number, special_chars_in_block))
            })
    }

    fn get_numbers_in_matrix(&self) -> impl Iterator<Item = Coordinates> + '_ {
        self.matrix
            .iter()
            .map(|line| {
                line.iter().enumerate().filter_map(|(column, char)| {
                    let previous_char = column
                        .checked_sub(1)
                        .and_then(|prev_col| line.get(prev_col));

                    if !char.is_ascii_digit() || previous_char.is_some_and(|c| c.is_ascii_digit()) {
                        None
                    } else {
                        Some(column)
                    }
                })
            })
            .enumerate()
            .flat_map(|(line, columns)| columns.map(move |column| (line, column)))
    }

    fn get_number_at_index(&self, line: usize, col: usize) -> Option<String> {
        self.matrix.get(line)?.get(col..).map(|slice| {
            slice
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .cloned()
                .collect::<String>()
        })
    }

    fn get_chars_in_block(
        &self,
        start_line: usize,
        end_line: usize,
        start_col: usize,
        end_col: usize,
    ) -> impl Iterator<Item = (usize, usize, char)> + '_ {
        self.matrix
            .iter()
            .enumerate()
            .skip(start_line)
            .take(end_line - start_line + 1)
            .flat_map(move |(line_index, line)| {
                line.iter()
                    .cloned()
                    .enumerate()
                    .skip(start_col)
                    .take(end_col - start_col + 1)
                    .map(move |(col_index, char)| (line_index, col_index, char))
                    .filter(|(_, _, c)| !c.is_ascii_digit() && !c.eq(&'.'))
            })
    }
}
