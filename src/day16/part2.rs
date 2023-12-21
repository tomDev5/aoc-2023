use itertools::Itertools;

use crate::mirror_maze::MirrorMaze;

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

    println!(
        "{:?}",
        mirror_maze.get_maximal_number_of_passed_coordinates()
    );
}
