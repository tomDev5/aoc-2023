use itertools::Itertools;

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

    println!("result: {:?}", picks(&corners));
}

fn picks(corners: &Vec<(isize, isize)>) -> isize {
    let outline_length = corners
        .iter()
        .fold((0, (0, 0)), |(length, previous), point| {
            (
                length
                    + isize::abs_diff(previous.0, point.0)
                    + isize::abs_diff(previous.1, point.1),
                *point,
            )
        })
        .0 as isize;
    shoelace(corners) - (outline_length / 2) + 1 + outline_length
}

fn shoelace(corners: &Vec<(isize, isize)>) -> isize {
    let mut area = 0isize;

    for i in 0..corners.len() {
        let j = (i + 1) % corners.len();
        area += corners[i].0 * corners[j].1;
        area -= corners[j].0 * corners[i].1;
    }

    area.abs() / 2
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::U),
            'D' => Ok(Self::D),
            'L' => Ok(Self::L),
            'R' => Ok(Self::R),
            _ => Err(()),
        }
    }
}
