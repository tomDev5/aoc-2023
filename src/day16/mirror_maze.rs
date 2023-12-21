use std::collections::HashSet;

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
    pub fn get_maximal_number_of_passed_coordinates(&self) -> Option<usize> {
        [
            (0..self.maze.len())
                .map(|line_index| {
                    self.get_passed_coordinates(
                        &mut HashSet::new(),
                        Coordinates::new(line_index, 0),
                        Direction::Right,
                    )
                    .len()
                })
                .max()?,
            (0..self.maze.len())
                .map(|line_index| {
                    self.get_passed_coordinates(
                        &mut HashSet::new(),
                        Coordinates::new(line_index, self.maze[line_index].len() - 1),
                        Direction::Right,
                    )
                    .len()
                })
                .max()?,
            (0..self.maze[0].len())
                .map(|column_index| {
                    self.get_passed_coordinates(
                        &mut HashSet::new(),
                        Coordinates::new(0, column_index),
                        Direction::Down,
                    )
                    .len()
                })
                .max()?,
            (0..self.maze[0].len())
                .map(|column_index| {
                    self.get_passed_coordinates(
                        &mut HashSet::new(),
                        Coordinates::new(self.maze.len() - 1, column_index),
                        Direction::Down,
                    )
                    .len()
                })
                .max()?,
        ]
        .into_iter()
        .max()
    }

    pub fn get_passed_coordinates(
        &self,
        visitation_log: &mut HashSet<(Coordinates, Direction)>,
        from: Coordinates,
        direction: Direction,
    ) -> Vec<Coordinates> {
        let mut stack = Vec::new();
        stack.push((from, direction));

        let mut passed_coordinates = vec![];

        while let Some((current_coordinates, current_direction)) = stack.pop() {
            if visitation_log.contains(&(current_coordinates, current_direction)) {
                continue;
            }

            visitation_log.insert((current_coordinates, current_direction));

            if let Some(element) = self.get(current_coordinates) {
                passed_coordinates.push(current_coordinates);

                for (next_direction, next_coordinates) in element
                    .get_directions(current_direction)
                    .into_iter()
                    .filter_map(|d| Some((d, current_coordinates.move_to(&d)?)))
                {
                    stack.push((next_coordinates, next_direction));
                }
            }
        }

        passed_coordinates.into_iter().unique().collect_vec()
    }

    fn get(&self, coordinates: Coordinates) -> Option<Element> {
        if coordinates.0 < self.maze.len() && coordinates.1 < self.maze[0].len() {
            Some(self.maze[coordinates.0][coordinates.1])
        } else {
            None
        }
    }
}
