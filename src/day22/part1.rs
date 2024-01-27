use std::collections::{hash_map::Entry, HashMap, HashSet};

use itertools::{iproduct, Itertools};

const INPUT: &str = include_str!("../../data/day22/input.txt");

fn main() {
    let bricks = INPUT
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

    let (x_len, y_len, z_len) = bricks
        .clone()
        .flatten()
        .fold((0, 0, 0), |(max_x, max_y, max_z), (x, y, z)| {
            (max_x.max(x + 1), max_y.max(y + 1), max_z.max(z + 1))
        });

    let mut map: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; y_len]; x_len]; z_len];
    for (brick_id, brick) in bricks.enumerate().map(|(i, b)| (i + 1, b)) {
        for point in brick {
            map[point.2][point.0][point.1] = brick_id; // map is [z,y,x]
        }
    }

    while {
        let mut moves = 0;
        for z in 2..z_len {
            let mut brick_id_to_points = HashMap::new();
            for (x, y) in iproduct!(0..map[z].len(), 0..map[z].len()) {
                let brick_id = map[z][x][y];
                if brick_id == 0 {
                    continue;
                }
                let points_list_for_brick_id = match brick_id_to_points.entry(brick_id) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(HashSet::new()),
                };
                points_list_for_brick_id.insert((x, y));
            }

            for (brick_id, points) in brick_id_to_points {
                if points.iter().all(|(x, y)| map[z - 1][*x][*y] == 0) {
                    moves += 1;
                    points.into_iter().for_each(|(x, y)| {
                        map[z - 1][x][y] = brick_id;
                        map[z][x][y] = 0;
                    })
                }
            }
        }
        moves > 0
    } {}

    let mut brick_to_supporting_bricks = HashMap::new();
    for (z, x, y) in iproduct!(1..z_len, 0..x_len, 0..y_len) {
        let brick_id = map[z][x][y];
        if brick_id == 0 {
            continue;
        }
        let bricks_below = match brick_to_supporting_bricks.entry(brick_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(HashSet::new()),
        };
        let brick_below_id = map[z - 1][x][y];
        if brick_below_id != 0 && brick_below_id != brick_id {
            bricks_below.insert(brick_below_id);
        }
    }

    let mut brick_to_supported_bricks = HashMap::new();
    for (z, x, y) in iproduct!(1..z_len - 1, 0..x_len, 0..y_len) {
        let brick_id = map[z][x][y];
        if brick_id == 0 {
            continue;
        }
        let bricks_below = match brick_to_supported_bricks.entry(brick_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(HashSet::new()),
        };
        let brick_above_id = map[z + 1][x][y];
        if brick_above_id != 0 && brick_above_id != brick_id {
            bricks_below.insert(brick_above_id);
        }
    }

    let r = (1..INPUT.lines().count() + 1)
        .filter(|brick_id| {
            let Some(supported_bricks) = brick_to_supported_bricks.get(brick_id) else {
                return true;
            };
            supported_bricks.iter().all(|supported_brick| {
                let Some(supporting_bricks) = brick_to_supporting_bricks.get(supported_brick)
                else {
                    panic!("Invalid");
                };
                supporting_bricks.len() > 1
            })
        })
        .collect_vec();

    println!("result: {:?}", r.len());
}
