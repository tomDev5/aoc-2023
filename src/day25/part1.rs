use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../data/day25/input.txt");

fn main() {
    let mut connections = INPUT
        .lines()
        .filter_map(|line| line.split(':').collect_tuple())
        .map(|(from, to)| {
            to.split_whitespace()
                .map(move |to| vec![(from, to), (to, from)])
        })
        .flatten()
        .flatten()
        .sorted_by_key(|(from, _)| *from)
        .group_by(|(from, _)| *from)
        .into_iter()
        .map(|(from, to)| (from, to.map(|(_, to)| to).collect_vec()))
        .collect::<HashMap<_, _>>();

    let vertices = connections.keys().cloned().collect::<Vec<_>>();
    let costs = connections
        .iter()
        .map(|(from, to)| to.into_iter().map(move |to| ((*from, *to), 1isize)))
        .flatten()
        .collect::<Vec<_>>();

    let mut minimal_cuts = Vec::new();
    let mut source = "";
    let mut sink = "";
    for sinkstart in connections.keys().combinations(2) {
        let current_source = sinkstart[0];
        let curret_sink = sinkstart[1];

        let max_flow = pathfinding::directed::edmonds_karp::edmonds_karp_dense(
            &vertices,
            current_source,
            curret_sink,
            costs.clone(),
        );

        if max_flow.1 == 3 {
            minimal_cuts = max_flow.2.into_iter().map(|(point, _)| point).collect_vec();
            source = current_source;
            sink = curret_sink;
            break;
        }
    }

    for minimal_cut in minimal_cuts {
        let Some(tos) = connections.get_mut(&minimal_cut.0) else {
            continue;
        };
        let Some(position) = tos.iter().position(|to| to == &minimal_cut.1) else {
            continue;
        };
        tos.remove(position);
        let Some(tos) = connections.get_mut(&minimal_cut.1) else {
            continue;
        };
        let Some(position) = tos.iter().position(|to| to == &minimal_cut.0) else {
            continue;
        };
        tos.remove(position);
    }

    let source_group = pathfinding::directed::bfs::bfs_reach(source, |point| {
        connections.get(point).unwrap().clone()
    });

    let sink_group = pathfinding::directed::bfs::bfs_reach(sink, |point| {
        connections.get(point).unwrap().clone()
    });

    println!("{}", source_group.count() * sink_group.count());
}
