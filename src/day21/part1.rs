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

    let number_of_steps = 64;

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
        if line > 0 {
            moves.push(((line - 1, column), distance + 1));
        }
        if line < map.len() - 1 {
            moves.push(((line + 1, column), distance + 1));
        }
        if column > 0 {
            moves.push(((line, column - 1), distance + 1));
        }
        if column < map[0].len() - 1 {
            moves.push(((line, column + 1), distance + 1));
        }
        moves
            .into_iter()
            .filter(|((line, column), _)| map[*line][*column] != Element::Rock)
            .collect_vec()
    })
    .filter(|(_, distance)| *distance == steps)
    .map(|(position, _)| position)
    .sorted()
    .unique()
    .collect_vec()
}
