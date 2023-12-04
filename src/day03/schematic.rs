use itertools::Itertools;

pub struct Schematic {
    matrix: Vec<Vec<char>>,
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        Self {
            matrix: input.lines().map(|s| s.chars().collect_vec()).collect_vec(),
        }
    }

    pub fn get_part_numbers(&self) -> impl Iterator<Item = usize> + '_ {
        self.get_numbers_in_matrix()
            .filter_map(|(line, col)| {
                self.get_number_at_index(line, col)
                    .map(|number_str| (line, col, number_str))
            })
            .filter(|(line, col, number_str)| {
                let start_line = line.checked_sub(1).unwrap_or(0);
                let end_line = line + 1;
                let start_col = col.checked_sub(1).unwrap_or(0);
                let end_col = col + number_str.len();

                let special_chars_in_block = self
                    .get_chars_in_block(start_line, end_line, start_col, end_col)
                    .filter(|c| !c.is_digit(10) && *c != '.')
                    .count();
                special_chars_in_block > 0
            })
            .filter_map(|(_, _, number_str)| number_str.parse::<usize>().ok())
    }

    fn get_numbers_in_matrix(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.matrix
            .iter()
            .map(|line| {
                line.iter().enumerate().filter_map(|(column, char)| {
                    let previous_char = column
                        .checked_sub(1)
                        .and_then(|prev_col| line.get(prev_col));

                    if !char.is_digit(10) || previous_char.is_some_and(|c| c.is_digit(10)) {
                        None
                    } else {
                        Some(column)
                    }
                })
            })
            .enumerate()
            .map(|(line, columns)| columns.map(move |column| (line, column)))
            .flatten()
    }

    fn get_number_at_index(&self, line: usize, col: usize) -> Option<String> {
        self.matrix.get(line)?.get(col..).map(|slice| {
            slice
                .into_iter()
                .take_while(|c| c.is_digit(10))
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
    ) -> impl Iterator<Item = char> + '_ {
        self.matrix
            .iter()
            .skip(start_line)
            .take(end_line - start_line + 1)
            .map(move |line| line.iter().skip(start_col).take(end_col - start_col + 1))
            .flatten()
            .cloned()
    }
}
