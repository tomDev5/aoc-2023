use itertools::Itertools;
use map::Map;

use crate::tile::Tile;

mod map;
mod tile;

const INPUT: &str = include_str!("../../data/day23/input.txt");

fn main() {
    let map = Map::new(INPUT);

    let start = map.get_first_position().expect("Missing start");
    let end = map.get_last_position().expect("Missing end");

    let path = pathfinding::directed::yen::yen(
        &start,
        |(y, x)| {
            let (y, x) = (*y, *x);
            match map.get((y, x)).expect("Bad coordinates") {
                Tile::UpSlope => vec![(y - 1, x)],
                Tile::DownSlope => vec![(y + 1, x)],
                Tile::LeftSlope => vec![(y, x - 1)],
                Tile::RightSlope => vec![(y, x + 1)],
                Tile::Path => vec![(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
                    .into_iter()
                    .filter(|point| map.get(*point).is_some_and(|tile| *tile != Tile::Forest))
                    .collect_vec(),
                Tile::Forest => unreachable!("Cannot pass through forest"),
            }
            .into_iter()
            .map(|point| (point, 1))
            .collect_vec()
        },
        |point| *point == end,
        usize::MAX,
    );

    println!("result: {:?}", path.last().expect("No paths").1);
}
