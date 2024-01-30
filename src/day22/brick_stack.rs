use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

#[derive(Clone)]
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

    pub fn move_bricks(&mut self) -> impl Iterator<Item = usize> + '_ {
        let mut moved_bricks = HashSet::new();
        loop {
            let new_moved_bricks = self.move_bricks_one_tick().collect_vec();
            if new_moved_bricks.is_empty() {
                break;
            }
            moved_bricks.extend(new_moved_bricks);
        }
        moved_bricks.into_iter()
    }

    pub fn move_bricks_one_tick(&mut self) -> impl Iterator<Item = usize> + '_ {
        let mut moved_bricks = HashSet::new();
        for z in 2..self.inner.len() {
            let brick_id_to_points = self.get_points_of_bricks(z);

            for (brick_id, points) in brick_id_to_points {
                if points.iter().all(|(x, y)| self.is_empty_below(z, *x, *y)) {
                    moved_bricks.insert(brick_id);
                    self.move_brick_down(z, brick_id, points);
                }
            }
        }
        moved_bricks.into_iter()
    }

    #[allow(dead_code)]
    pub fn get_disintegratable_bricks(&self) -> impl Iterator<Item = usize> + '_ {
        let brick_to_supporting_bricks = self.get_brick_diff(-1);
        let brick_to_supported_bricks = self.get_brick_diff(1);

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

    #[allow(dead_code)]
    pub fn get_bricks_and_collateral(&self) -> HashMap<usize, HashSet<usize>> {
        let mut removed_brick_to_moved_bricks = HashMap::<usize, HashSet<usize>>::new();
        for brick_id in self.iterate_bricks() {
            let mut new_stack = self.clone();
            new_stack.remove_brick(brick_id);
            removed_brick_to_moved_bricks.insert(brick_id, new_stack.move_bricks().collect());
        }
        removed_brick_to_moved_bricks
    }

    fn get_points_of_bricks(&self, z: usize) -> HashMap<usize, HashSet<(usize, usize)>> {
        let mut brick_id_to_points = HashMap::new();
        for (x, y) in iproduct!(0..self.inner[z].len(), 0..self.inner[z].len()) {
            if let Some(brick_id) = self.get(z, x, y) {
                brick_id_to_points
                    .entry(brick_id)
                    .or_insert_with(HashSet::new)
                    .insert((x, y));
            }
        }
        brick_id_to_points
    }

    fn is_empty_below(&self, z: usize, x: usize, y: usize) -> bool {
        self.get(z - 1, x, y).is_none()
    }

    fn move_brick_down(&mut self, z: usize, brick_id: usize, points: HashSet<(usize, usize)>) {
        for (x, y) in points {
            self.inner[z - 1][x][y] = Some(brick_id);
            self.inner[z][x][y] = None;
        }
    }

    fn get_brick_diff(&self, diff: isize) -> HashMap<usize, HashSet<usize>> {
        self.iterate_indexes()
            .filter_map(|(z, x, y)| {
                Some((
                    self.get(z, x, y)?,
                    self.get(z.checked_add_signed(diff).expect("Invalid diff"), x, y)?,
                ))
            })
            .filter(|(brick_id, brick_above_id)| brick_id != brick_above_id)
            .sorted_by_key(|(brick_id, _)| *brick_id)
            .group_by(|(brick_id, _)| *brick_id)
            .into_iter()
            .map(|(brick_id, group)| (brick_id, group.map(|(_, brick_id)| brick_id).collect()))
            .collect()
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

    fn get(&self, z: usize, x: usize, y: usize) -> Option<usize> {
        *self.inner.get(z)?.get(x)?.get(y)?
    }

    fn remove_brick(&mut self, brick_id: usize) {
        let mut exists = false;
        for layer in &mut self.inner {
            for line in layer {
                for column in line {
                    if column.is_some_and(|map_brick_id| map_brick_id == brick_id) {
                        *column = None;
                        exists = true;
                    }
                }
            }
        }
        if exists {
            self.number_of_bricks -= 1;
        }
    }
}
