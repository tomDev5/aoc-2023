use std::collections::{hash_map::Entry, HashMap, HashSet};

use itertools::{iproduct, Itertools};

pub struct BrickStack {
    inner: Vec<Vec<Vec<Option<usize>>>>,
    number_of_bricks: usize,
}

impl BrickStack {
    pub fn new(input: &str) -> Self {
        let bricks = input
            .lines()
            .map(|line| line.split('~'))
            .filter_map(|coordinates| {
                coordinates
                    .filter_map(|coordinate| {
                        coordinate
                            .split(',')
                            .filter_map(|number| number.parse::<usize>().ok())
                            .collect_tuple()
                    })
                    .collect_tuple()
            })
            .map(|((x1, y1, z1), (x2, y2, z2))| iproduct!(x1..=x2, y1..=y2, z1..=z2));
        let (x_len, y_len, z_len) =
            bricks
                .clone()
                .flatten()
                .fold((0, 0, 0), |(max_x, max_y, max_z), (x, y, z)| {
                    (max_x.max(x + 1), max_y.max(y + 1), max_z.max(z + 1))
                });

        let mut map: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; y_len]; x_len]; z_len];
        for (brick_id, brick) in bricks.enumerate() {
            for point in brick {
                map[point.2][point.0][point.1] = Some(brick_id); // map is [z,y,x]
            }
        }

        Self {
            inner: map,
            number_of_bricks: input.lines().count(),
        }
    }

    pub fn move_bricks(&mut self) {
        while self.move_bricks_one_tick() > 0 {}
    }

    pub fn move_bricks_one_tick(&mut self) -> usize {
        let mut moves = 0;
        for z in 2..self.inner.len() {
            let brick_id_to_points = self.get_points_of_brick(z);

            for (brick_id, points) in brick_id_to_points {
                if points.iter().all(|(x, y)| self.is_empty_below(z, *x, *y)) {
                    moves += 1;
                    self.move_bricks_up(z, brick_id, points);
                }
            }
        }
        moves
    }

    pub fn get_disintegratable_bricks(&self) -> impl Iterator<Item = usize> + '_ {
        let brick_to_supporting_bricks = self.brick_to_supporting_bricks();
        let brick_to_supported_bricks = self.brick_to_supported_bricks();

        self.iterate_bricks().filter(move |brick_id| {
            let Some(supported_bricks) = brick_to_supported_bricks.get(brick_id) else {
                return true;
            };
            supported_bricks.iter().all(|supported_brick| {
                let Some(supporting_bricks) = brick_to_supporting_bricks.get(supported_brick)
                else {
                    unreachable!("Invalid - brick_to_supporting_bricks not matching brick_to_supported_bricks");
                };
                supporting_bricks.len() > 1
            })
        })
    }

    fn get_points_of_brick(&self, z: usize) -> HashMap<usize, HashSet<(usize, usize)>> {
        let mut brick_id_to_points = HashMap::new();
        for (x, y) in iproduct!(0..self.inner[z].len(), 0..self.inner[z].len()) {
            if let Some(brick_id) = self.inner[z][x][y] {
                brick_id_to_points
                    .entry(brick_id)
                    .or_insert_with(HashSet::new)
                    .insert((x, y));
            }
        }
        brick_id_to_points
    }

    fn is_empty_below(&self, z: usize, x: usize, y: usize) -> bool {
        self.inner[z - 1][x][y].is_none()
    }

    fn move_bricks_up(&mut self, z: usize, brick_id: usize, points: HashSet<(usize, usize)>) {
        for (x, y) in points {
            self.inner[z - 1][x][y] = Some(brick_id);
            self.inner[z][x][y] = None;
        }
    }

    fn brick_to_supporting_bricks(&self) -> HashMap<usize, HashSet<usize>> {
        let mut brick_to_supporting_bricks = HashMap::new();
        for (z, x, y) in self.iterate_indexes() {
            if let Some(brick_id) = self.inner[z][x][y] {
                let bricks_below = match brick_to_supporting_bricks.entry(brick_id) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(HashSet::new()),
                };
                if let Some(brick_below_id) = self.inner[z - 1][x][y] {
                    if brick_below_id != brick_id {
                        bricks_below.insert(brick_below_id);
                    }
                }
            }
        }
        brick_to_supporting_bricks
    }

    fn brick_to_supported_bricks(&self) -> HashMap<usize, HashSet<usize>> {
        let mut brick_to_supported_bricks = HashMap::new();
        for (z, x, y) in self.iterate_indexes() {
            if let Some(brick_id) = self.inner[z][x][y] {
                let bricks_below = match brick_to_supported_bricks.entry(brick_id) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(HashSet::new()),
                };
                if let Some(brick_above_id) = self.inner[z + 1][x][y] {
                    if brick_above_id != brick_id {
                        bricks_below.insert(brick_above_id);
                    }
                }
            }
        }
        brick_to_supported_bricks
    }

    fn iterate_indexes(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        iproduct!(
            1..self.inner.len(),
            0..self.inner[0].len(),
            0..self.inner[0][0].len()
        )
    }

    fn iterate_bricks(&self) -> impl Iterator<Item = usize> + '_ {
        0..self.number_of_bricks
    }
}
