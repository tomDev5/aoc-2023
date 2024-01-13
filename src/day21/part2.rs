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
        .expect("no starting position found");

    let number_of_steps = 6;

    println!(
        "{:?}",
        get_possible_ending_positions(&map, starting_position, number_of_steps).len()
    );
}

fn get_possible_ending_positions(
    map: &Vec<Vec<Element>>,
    starting_position: (usize, usize),
    steps: usize,
) -> Vec<(usize, usize)> {
    pathfinding::directed::bfs::bfs_reach((starting_position, 0), |((line, column), distance)| {
        let (line, column) = (*line, *column);
        if *distance == steps {
            return vec![];
        }
        let mut moves = vec![];

        let new_line = line.checked_sub(1).unwrap_or(map.len() - 1);
        let new_move = (new_line, column);
        if map[new_move.0][new_move.1] != Element::Rock {
            moves.push((new_move, distance + 1));
        }

        let new_line = (line + 1) % map.len();
        let new_move = (new_line, column);
        if map[new_move.0][new_move.1] != Element::Rock {
            moves.push((new_move, distance + 1));
        }

        let new_column = column.checked_sub(1).unwrap_or(map[0].len() - 1);
        let new_move = (line, new_column);
        if map[new_move.0][new_move.1] != Element::Rock {
            moves.push((new_move, distance + 1));
        }

        let new_column = (column + 1) % map[0].len();
        let new_move = (line, new_column);
        if map[new_move.0][new_move.1] != Element::Rock {
            moves.push((new_move, distance + 1));
        }
        moves
    })
    .filter(|(_, distance)| *distance == steps)
    .map(|(position, _)| position)
    .sorted()
    .unique()
    .collect_vec()
}
