use std::collections::HashSet;

use itertools::Itertools;

use crate::{coordinates::Coordinates, direction::Direction, mirror_maze::MirrorMaze};

mod coordinates;
mod direction;
mod element;
mod mirror_maze;

const INPUT: &str = include_str!("../../data/day16/input.txt");

fn main() {
    let mirror_maze = MirrorMaze::new(
        INPUT
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );

    let coordinates = mirror_maze.get_passed_coordinates(
        &mut HashSet::new(),
        Coordinates::new(0, 0),
        Direction::Right,
    );

    println!("{:?}", coordinates.len());
}
