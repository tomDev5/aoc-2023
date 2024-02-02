use map::Map;

use crate::point_and_previous::PointAndPrevious;

mod map;
mod point_and_previous;
mod tile;

const INPUT: &str = include_str!("../../data/day23/input.txt");

fn main() {
    let map = Map::new(INPUT);

    let start = map.get_first_position().expect("Missing start");
    let end = map.get_last_position().expect("Missing end");

    let path = pathfinding::directed::yen::yen(
        &PointAndPrevious::new((-1isize, -1isize), start),
        |PointAndPrevious { previous, point }| {
            map.get_next_condensed(*previous, *point, end)
                .map(|(next_point, cost)| (PointAndPrevious::new(*point, next_point), cost))
                .collect::<Vec<_>>()
        },
        |PointAndPrevious { previous: _, point }| *point == end,
        usize::MAX,
    );

    println!("result: {}", path.last().expect("No paths").1);
}
