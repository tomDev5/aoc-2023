use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::matrix::{directions::DIRECTIONS_4, Matrix};
use petgraph::{algo::all_simple_paths, Graph, Undirected};

use crate::tile::Tile;

mod point_and_previous;
mod tile;

const INPUT: &str = include_str!("../../data/day23/input.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| Tile::try_from(c).ok())
                .collect_vec()
        })
        .collect::<Matrix<Tile>>();

    let start = (0, 1);
    let end = (grid.rows - 1, grid.columns - 2);

    let successors = |point: (usize, usize)| -> Vec<(usize, usize)> {
        match grid.get(point) {
            Some(&Tile::Forest) => vec![],
            Some(_) => DIRECTIONS_4
                .iter()
                .filter_map(|d| grid.move_in_direction(point, *d))
                .filter(|p| grid.get(*p) != Some(&Tile::Forest))
                .collect(),
            None => panic!("Bad coordinates"),
        }
    };

    let mut graph = Graph::<(usize, usize), i32, Undirected>::new_undirected();
    let point_to_node_index: HashMap<_, _> = grid
        .items()
        .filter(|(p, t)| {
            **t != Tile::Forest && (successors(*p).len() > 2 || *p == start || *p == end)
        })
        .map(|(p, _)| (p, graph.add_node(p)))
        .collect();

    for &point in point_to_node_index.keys() {
        let mut seen = HashSet::new();
        for _ in 0..successors(point).len() {
            let path = pathfinding::directed::bfs::bfs(
                &point,
                |neighbor| {
                    successors(*neighbor).into_iter().filter(|&n| {
                        !seen.contains(&n)
                            && (!point_to_node_index.contains_key(&n)
                                || !graph.contains_edge(
                                    *point_to_node_index.get(&n).unwrap(),
                                    *point_to_node_index.get(&point).unwrap(),
                                ))
                    })
                },
                |neighbor| {
                    let valid_neighbor = point_to_node_index.contains_key(neighbor)
                        && !seen.contains(neighbor)
                        && !graph.contains_edge(
                            *point_to_node_index.get(neighbor).unwrap(),
                            *point_to_node_index.get(&point).unwrap(),
                        );

                    *neighbor != point && valid_neighbor
                },
            );
            if let Some(path) = path {
                let neighbor = *path.last().unwrap();
                seen.insert(neighbor);
                graph.add_edge(
                    *point_to_node_index.get(&point).unwrap(),
                    *point_to_node_index.get(&neighbor).unwrap(),
                    path.len() as i32 - 1,
                );
            }
        }
    }

    println!(
        "{:?}",
        all_simple_paths::<Vec<_>, _>(
            &graph,
            *point_to_node_index.get(&start).unwrap(),
            *point_to_node_index.get(&end).unwrap(),
            0,
            None
        )
        .map(|path| {
            path.windows(2)
                .map(|w| graph.edges_connecting(w[0], w[1]).next().unwrap().weight())
                .sum::<i32>()
        })
        .max()
        .unwrap()
    );
}
