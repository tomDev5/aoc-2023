use crate::direction::Direction;

mod direction;
mod math;

const INPUT: &str = include_str!("../../data/day18/input.txt");

fn main() {
    let corners = INPUT
        .lines()
        .filter_map(|line| line.split_ascii_whitespace().nth(2))
        .map(|line| &line[2..line.len() - 1])
        .filter_map(|line| {
            let direction = match line.chars().nth(5)? {
                '0' => Direction::R,
                '1' => Direction::D,
                '2' => Direction::L,
                '3' => Direction::U,
                _ => {
                    return None;
                }
            };
            let distance = isize::from_str_radix(&line[0..5], 16).ok()?;
            Some((direction, distance))
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
