use map::Map;

mod map;
mod tile;

const INPUT: &str = include_str!("../../data/day23/input.txt");

fn main() {
    let map = Map::new(INPUT);

    let start = map.get_first_position().expect("Missing start");
    let end = map.get_last_position().expect("Missing end");

    let path = pathfinding::directed::yen::yen(
        &start,
        |point| map.get_next(*point).map(|point| (point, 1)),
        |point| *point == end,
        usize::MAX,
    );

    println!("result: {:?}", path.last().expect("No paths").1);
}
