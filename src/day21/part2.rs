use itertools::Itertools;

use crate::element::Element;

const INPUT: &str = include_str!("../../data/day21/input.txt");

mod element;

fn main() {
    let map = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    if c == 'S' {
                        Some(Element::Garden)
                    } else {
                        Element::try_from(c).ok()
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    let starting_position = INPUT
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains('S'))
        .filter_map(|(i, line)| Some((i, line.find('S')?)))
        .next()
        .map(|(i, j)| (i as isize, j as isize))
        .expect("no starting position found");

    let number_of_steps = 26501365;

    let (u0, u1, u2) = (0..3)
        .map(|i| {
            get_possible_ending_positions(
                &map,
                starting_position,
                map[0].len() - 1 - starting_position.1 as usize + (map[0].len() * i),
            )
            .len()
        })
        .collect_tuple()
        .expect("Failed to get u");

    let c = u0;
    let a = (u2 - (2 * u1) + u0) / 2;
    let b = u1 - u0 - a;
    let n: usize = (number_of_steps - map.len() / 2) / map.len();
    println!("result: {:?}", (a * n.pow(2)) + (b * n) + c);
}

fn get_possible_ending_positions(
    map: &Vec<Vec<Element>>,
    starting_position: (isize, isize),
    steps: usize,
) -> Vec<(isize, isize)> {
    pathfinding::directed::bfs::bfs_reach((starting_position, 0), |((line, column), distance)| {
        let (line, column) = (*line, *column);
        if *distance == steps {
            return vec![];
        }

        vec![
            ((line - 1, column), distance + 1),
            (((line + 1), column), distance + 1),
            ((line, column - 1), distance + 1),
            ((line, column + 1), distance + 1),
        ]
        .into_iter()
        .filter(|((line, column), _)| {
            let (line, column) = (
                line.rem_euclid(map.len() as isize),
                column.rem_euclid(map[0].len() as isize),
            );
            map[line as usize][column as usize] != Element::Rock
        })
        .collect_vec()
    })
    .filter(|(_, distance)| *distance == steps)
    .map(|(position, _)| position)
    .collect_vec()
}
