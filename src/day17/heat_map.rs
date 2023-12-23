use pathfinding::matrix::{directions, Matrix};

pub struct HeatMap {
    inner: Matrix<u32>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: u64,
}

impl HeatMap {
    pub fn new(map: impl Iterator<Item = impl Iterator<Item = u32>>) -> Self {
        Self {
            inner: map.collect(),
        }
    }

    pub fn get_minimal_heat_loss<const MIN: u64, const MAX: u64>(&self) -> Option<u32> {
        let start = State {
            position: (0, 0),
            direction: (0, 0),
            distance: 0,
        };

        let end = (self.inner.rows - 1, self.inner.columns - 1);

        let path = pathfinding::directed::astar::astar(
            &start,
            |state| match state.distance >= MIN
                || (state.direction.0 == 0 && state.direction.1 == 0)
            {
                true => self.get_neighbors::<MAX>(state, &start),
                false => self.get_next_successor::<MIN>(state),
            },
            |state| (end.0.abs_diff(state.position.0) + end.1.abs_diff(state.position.1)) as u32,
            |state| state.position == end && state.distance >= MIN,
        );

        path.map(|path| path.1)
    }

    fn get_neighbors<const MAX: u64>(&self, state: &State, start: &State) -> Vec<(State, u32)> {
        [directions::N, directions::S, directions::E, directions::W]
            .iter()
            .flat_map(|direction| {
                self.inner
                    .move_in_direction(state.position, *direction)
                    .map(|point| {
                        (
                            point,
                            *direction,
                            *self.inner.get(point).expect("Point to exist"),
                        )
                    })
            })
            .filter(|(position, direction, _)| {
                let is_direction_inverse =
                    state.direction.0 == -direction.0 && state.direction.1 == -direction.1;

                !is_direction_inverse && *position != start.position
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

    fn get_next_successor<const MIN: u64>(&self, state: &State) -> Vec<(State, u32)> {
        match self
            .inner
            .move_in_direction(state.position, state.direction)
        {
            Some(point) => {
                let weight = *self.inner.get(point).expect("Point to exist");
                let new_state = State {
                    position: point,
                    direction: state.direction,
                    distance: state.distance + 1,
                };

                vec![(new_state, weight)]
            }
            None => Vec::with_capacity(0),
        }
    }
}
