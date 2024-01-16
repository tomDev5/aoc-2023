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

    let number_of_steps = 26501365;

    let (u0, u1, u2) = (0..3)
        .map(|i| {
            get_possible_ending_positions(
                &map,
                starting_position,
                map[0].len() - 1 - starting_position.1 + (map[0].len() * i),
            )
            .len()
        })
        .collect_tuple()
        .expect("Failed to get u");

    println!(
        "u0: {:?} (should be 3821), u1: {:?} (should be 34234), u2: {:?} (should be 94963)",
        u0, u1, u2
    );
    let c = u0;
    let a = (u2 - (2 * u1) + u0) / 2;
    let b = u1 - u0 - a;
    let n: usize = (number_of_steps - 65) / 131;
    println!("n: {n}");
    println!("{:?}", (a * n.pow(2)) + (b * n) + c);
}

fn get_possible_ending_positions(
    map: &Vec<Vec<Element>>,
    starting_position: (usize, usize),
    steps: usize,
) -> Vec<(usize, usize)> {
    pathfinding::directed::bfs::bfs_reach(
        (starting_position, (0isize, 0isize), 0),
        |((line, column), (board_line, board_column), distance)| {
            let (line, column) = (*line, *column);
            let (board_line, board_column) = (*board_line, *board_column);
            if *distance == steps {
                return vec![];
            }
            let mut moves = vec![];

            moves.push((
                (line.checked_sub(1).unwrap_or(map.len() - 1), column),
                (
                    if line == 0 {
                        board_line - 1
                    } else {
                        board_line
                    },
                    board_column,
                ),
                distance + 1,
            ));
            moves.push((
                ((line + 1) % map.len(), column),
                (
                    if line == map.len() - 1 {
                        board_line + 1
                    } else {
                        board_line
                    },
                    board_column,
                ),
                distance + 1,
            ));
            moves.push((
                (line, column.checked_sub(1).unwrap_or(map[0].len() - 1)),
                (
                    board_line,
                    if column == 0 {
                        board_column - 1
                    } else {
                        board_column
                    },
                ),
                distance + 1,
            ));

            moves.push((
                (line, (column + 1) % map[0].len()),
                (
                    board_line,
                    if column == map[0].len() - 1 {
                        board_column + 1
                    } else {
                        board_column
                    },
                ),
                distance + 1,
            ));

            moves
                .into_iter()
                .filter(|((line, column), _, _)| map[*line][*column] != Element::Rock)
                .collect_vec()
        },
    )
    .filter(|(_, _, distance)| *distance == steps)
    .map(|(position, _, _)| position)
    .collect_vec()
}
