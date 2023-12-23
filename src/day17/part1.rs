use itertools::Itertools;

use crate::dijkstra::{dijkstra_path, Coordinates};

mod dijkstra;

const INPUT: &str = include_str!("../../data/day17/input.txt");

fn main() {
    let heat_map = INPUT
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec();

    let result = dijkstra_path(
        &heat_map,
        Coordinates::new(0, 0),
        Coordinates::new(heat_map.len() - 1, heat_map[0].len() - 1),
    );
    println!("result: {:?}", result)
}
