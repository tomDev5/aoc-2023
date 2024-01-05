use itertools::Itertools;

use crate::direction::Direction;

mod direction;
mod math;

const INPUT: &str = include_str!("../../data/day18/input.txt");

fn main() {
    let corners = INPUT
        .lines()
        .filter_map(|line| line.split_ascii_whitespace().take(2).collect_tuple())
        .filter_map(|(direction, distance)| {
            Some((
                Direction::try_from(direction.chars().next()?).ok()?,
                distance.parse::<isize>().ok()?,
            ))
        })
        .fold(vec![(0, 0)], |corners, current_move| {
            let previous_corner = *corners.last().unwrap();
            let mut corners = corners.clone();
            corners.push(match current_move.0 {
                Direction::U => (previous_corner.0 - current_move.1, previous_corner.1),
                Direction::D => (previous_corner.0 + current_move.1, previous_corner.1),
                Direction::L => (previous_corner.0, previous_corner.1 - current_move.1),
                Direction::R => (previous_corner.0, previous_corner.1 + current_move.1),
            });
            corners
        });

    println!("result: {:?}", math::picks(&corners));
}
