use itertools::Itertools;

pub struct Maze {
    matrix: Vec<Vec<char>>,
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            writeln!(f, "{}", row.iter().join(""))?;
        }
        Ok(())
    }
}

impl Maze {
    pub fn new(input: &str) -> Self {
        Self {
            matrix: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => ' ',
                            '-' => '─',
                            '|' => '│',
                            'F' => '┌',
                            'J' => '┘',
                            'L' => '└',
                            '7' => '┐',
                            _ => c,
                        })
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    pub fn get_loop(&self) -> Option<Vec<(usize, usize)>> {
        let mut points: Vec<(usize, usize)> = Vec::new();
        let start = self.find_start()?;
        let mut previous = start;
        let mut cursor = start;
        let mut loop_length = 0;
        while cursor != start || loop_length == 0 {
            let next_move = self
                .get_possible_moves(cursor)
                .into_iter()
                .filter(|pos| *pos != previous)
                .next()?;
            points.push(cursor);
            previous = cursor;
            cursor = next_move;
            loop_length += 1;
        }
        Some(points)
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        let line_length = self.matrix[0].len();
        let start_position = self.matrix.iter().flatten().position(|c| *c == 'S')?;
        Some((start_position / line_length, start_position % line_length))
    }

    fn get_possible_moves(
        &self,
        cursor: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let theoretical = self.get_element_possible_moves(cursor);
        theoretical.into_iter().filter(move |next_step| {
            self.get_element_possible_moves(*next_step)
                .contains(&cursor)
        })
    }

    fn get_element_possible_moves(
        &self,
        cursor: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        match self.get(cursor).unwrap() {
            '─' => {
                vec![
                    self.move_diff(cursor, (0, -1)),
                    self.move_diff(cursor, (0, 1)),
                ]
            }
            '│' => {
                vec![
                    self.move_diff(cursor, (-1, 0)),
                    self.move_diff(cursor, (1, 0)),
                ]
            }
            '┌' => {
                vec![
                    self.move_diff(cursor, (0, 1)),
                    self.move_diff(cursor, (1, 0)),
                ]
            }
            '┘' => {
                vec![
                    self.move_diff(cursor, (0, -1)),
                    self.move_diff(cursor, (-1, 0)),
                ]
            }
            '└' => {
                vec![
                    self.move_diff(cursor, (0, 1)),
                    self.move_diff(cursor, (-1, 0)),
                ]
            }
            '┐' => {
                vec![
                    self.move_diff(cursor, (0, -1)),
                    self.move_diff(cursor, (1, 0)),
                ]
            }
            'S' => {
                vec![
                    self.move_diff(cursor, (0, 1)),
                    self.move_diff(cursor, (0, -1)),
                    self.move_diff(cursor, (-1, 0)),
                    self.move_diff(cursor, (1, 0)),
                ]
            }
            _ => unreachable!(),
        }
        .into_iter()
        .filter_map(std::convert::identity)
    }

    fn get(&self, cursor: (usize, usize)) -> Option<&char> {
        self.matrix.get(cursor.0)?.get(cursor.1)
    }

    fn move_diff(&self, cursor: (usize, usize), diff: (isize, isize)) -> Option<(usize, usize)> {
        let cursor_isize = (cursor.0 as isize, cursor.1 as isize);
        if !(0..self.matrix.len() as isize).contains(&(cursor_isize.0 + diff.0))
            || !(0..self.matrix[0].len() as isize).contains(&(cursor_isize.1 + diff.1))
        {
            return None;
        }
        match (
            cursor.0.checked_add_signed(diff.0),
            cursor.1.checked_add_signed(diff.1),
        ) {
            (Some(line), Some(col)) => Some((line, col)),
            _ => None,
        }
    }
}
