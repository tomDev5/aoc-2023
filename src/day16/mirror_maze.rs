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

    pub fn get_passed_coordinates(
        &self,
        visitation_log: &mut HashSet<(Coordinates, Direction)>,
        from: Coordinates,
        direction: Direction,
    ) -> Vec<Coordinates> {
        let Some(element) = self.get(from) else {
            return vec![];
        };

        if visitation_log.contains(&(from, direction)) {
            return vec![];
        }

        visitation_log.insert((from, direction));

        element
            .get_directions(direction)
            .into_iter()
            .filter_map(|d| Some((d, from.move_to(&d)?)))
            .map(|(direction, coordinates)| {
                self.get_passed_coordinates(visitation_log, coordinates, direction)
            })
            .flatten()
            .chain(std::iter::once(from))
            .unique()
            .collect_vec()
    }

    fn get(&self, coordinates: Coordinates) -> Option<Element> {
        if coordinates.0 < self.maze.len() && coordinates.1 < self.maze[0].len() {
            Some(self.maze[coordinates.0][coordinates.1])
        } else {
            None
        }
    }
}
