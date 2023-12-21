use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;

use crate::{coordinates::Coordinates, direction::Direction, element::Element};

pub struct MirrorMaze {
    maze: Vec<Vec<Element>>,
}

impl MirrorMaze {
    pub fn new(maze: Vec<Vec<char>>) -> Self {
        MirrorMaze {
            maze: maze
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .filter_map(|i| Element::try_from(i).ok())
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn get_maximal_number_of_passed_coordinates(&self) -> usize {
        let left = (0..self.maze.len()).map(|line_index| {
            self.get_passed_coordinates(Coordinates::new(line_index, 0), Direction::Right)
                .count()
        });
        let right = (0..self.maze.len()).map(|line_index| {
            self.get_passed_coordinates(
                Coordinates::new(line_index, self.maze[0].len()),
                Direction::Right,
            )
            .count()
        });
        let top = (0..self.maze[0].len()).map(|column_index| {
            self.get_passed_coordinates(Coordinates::new(0, column_index), Direction::Right)
                .count()
        });
        let bottom = (0..self.maze[0].len()).map(|column_index| {
            self.get_passed_coordinates(
                Coordinates::new(self.maze.len() - 1, column_index),
                Direction::Right,
            )
            .count()
        });
        left.chain(right)
            .chain(top)
            .chain(bottom)
            .max()
            .unwrap_or(0)
    }

    pub fn get_passed_coordinates(
        &self,
        from: Coordinates,
        direction: Direction,
    ) -> impl Iterator<Item = Coordinates> + '_ {
        let mut stack = Vec::new();
        stack.push((from, direction));

        let mut visitation_log = HashSet::new();

        loop {
            let Some((current_coordinates, current_direction)) = stack.pop() else {
                break;
            };

            if visitation_log.contains(&(current_coordinates, current_direction)) {
                continue;
            }

            let Some(element) = self.get(current_coordinates) else {
                continue;
            };

            visitation_log.insert((current_coordinates, current_direction));
            stack.extend(
                element
                    .get_directions(current_direction)
                    .into_iter()
                    .filter_map(|d| Some((current_coordinates.move_to(&d)?, d))),
            );
        }

        visitation_log.into_iter().map(|(c, _d)| c).unique()
    }

    fn get(&self, coordinates: Coordinates) -> Option<Element> {
        if coordinates.0 < self.maze.len() && coordinates.1 < self.maze[0].len() {
            Some(self.maze[coordinates.0][coordinates.1])
        } else {
            None
        }
    }
}
