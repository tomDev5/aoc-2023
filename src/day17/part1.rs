use pathfinding::matrix::{directions, Matrix};

const INPUT: &str = include_str!("../../data/day17/input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: u64,
}

fn main() {
    let heat_map = INPUT
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .collect::<Matrix<_>>();

    println!("result: {:?}", get_minimal_heat_loss(&heat_map));
}

fn get_minimal_heat_loss(heat_map: &Matrix<u32>) -> Option<u32> {
    let start = State {
        position: (0, 0),
        direction: (100, 100), // fake
        distance: 0,
    };

    let end = (heat_map.rows - 1, heat_map.columns - 1);

    let path = pathfinding::directed::astar::astar(
        &start,
        |state| get_neighbors::<3>(state, &heat_map),
        |state| (end.0.abs_diff(state.position.0) + end.1.abs_diff(state.position.1)) as u32,
        |state| state.position == end,
    );

    path.map(|path| path.1)
}

fn get_neighbors<const MAX: u64>(state: &State, grid: &Matrix<u32>) -> Vec<(State, u32)> {
    [directions::N, directions::S, directions::E, directions::W]
        .iter()
        .flat_map(|direction| {
            grid.move_in_direction(state.position, *direction)
                .map(|point| (point, *direction, *grid.get(point).expect("Point to exist")))
        })
        .filter(|(_, direction, _)| {
            state.direction.0 != -direction.0 || state.direction.1 != -direction.1
        })
        .flat_map(|(position, direction, weight)| {
            let distance = match state.direction == direction {
                true => state.distance + 1,
                false => 1,
            };

            match distance <= MAX {
                true => {
                    let next_state = State {
                        position,
                        direction,
                        distance,
                    };
                    Some((next_state, weight))
                }
                false => None,
            }
        })
        .collect::<Vec<_>>()
}
