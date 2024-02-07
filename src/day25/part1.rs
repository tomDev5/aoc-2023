use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

const INPUT: &str = include_str!("../../data/day25/input.txt");

fn main() {
    let mut connections = INPUT
        .lines()
        .filter_map(|line| line.split(':').collect_tuple())
        .flat_map(|(from, to)| {
            to.split_whitespace()
                .flat_map(move |to| vec![(from, to), (to, from)])
        })
        .sorted_by_key(|(from, _)| *from)
        .group_by(|(from, _)| *from)
        .into_iter()
        .map(|(from, to)| (from, to.map(|(_, to)| to).collect::<HashSet<_>>()))
        .collect::<HashMap<_, _>>();

    let vertices = connections.keys().cloned().collect::<Vec<_>>();
    let costs = connections
        .iter()
        .flat_map(|(from, to)| to.iter().map(move |to| ((*from, *to), 1isize)))
        .collect::<Vec<_>>();

    let (source, sink, minimal_cuts) = iproduct!(connections.keys(), connections.keys())
        .map(|(source, sink)| {
            let max_flow = pathfinding::directed::edmonds_karp::edmonds_karp_dense(
                &vertices,
                source,
                sink,
                costs.clone(),
            );
            (
                *source,
                *sink,
                max_flow.2.into_iter().map(|(point, _)| point).collect_vec(),
            )
        })
        .find(|(_, _, minimal_cut)| minimal_cut.len() == 3)
        .expect("No minimal cut sized 3 found");

    for minimal_cut in minimal_cuts {
        connections
            .get_mut(&minimal_cut.0)
            .expect("Inavlid node")
            .remove(minimal_cut.1);
        connections
            .get_mut(&minimal_cut.1)
            .expect("Inavlid node")
            .remove(minimal_cut.0);
    }

    let source_group = pathfinding::directed::bfs::bfs_reach(source, |point| {
        connections.get(point).expect("Inavlid point").clone()
    });

    let sink_group = pathfinding::directed::bfs::bfs_reach(sink, |point| {
        connections.get(point).expect("Inavlid point").clone()
    });

    println!("{}", source_group.count() * sink_group.count());
}
