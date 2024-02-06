// credit to the excellant explanation by `ash42`:https://raw.githubusercontent.com/ash42/adventofcode/main/adventofcode2023/src/nl/michielgraat/adventofcode2023/day24/Day24.java

use hailstone::Hailstone;
use itertools::Itertools;

use crate::math::gaussian_elimination;

mod hailstone;
#[allow(dead_code)]
mod math;

const INPUT: &str = include_str!("../../data/day24/input.txt");

fn main() {
    let hailstones = INPUT
        .lines()
        .filter_map(|line| Hailstone::try_from(line).ok())
        .collect_vec();

    let mut coefficients = vec![vec![0.0; 4]; 4];
    let mut rhs = vec![0.0; 4];

    for i in 0..4 {
        let l1 = &hailstones[i];
        let l2 = &hailstones[i + 1];
        coefficients[i][0] = l2.velocity.1 - l1.velocity.1;
        coefficients[i][1] = l1.velocity.0 - l2.velocity.0;
        coefficients[i][2] = l1.position.1 - l2.position.1;
        coefficients[i][3] = l2.position.0 - l1.position.0;
        rhs[i] = -l1.position.0 * l1.velocity.1
            + l1.position.1 * l1.velocity.0
            + l2.position.0 * l2.velocity.1
            - l2.position.1 * l2.velocity.0;
    }

    let (x, y, vx, vy) = gaussian_elimination(coefficients, rhs)
        .into_iter()
        .collect_tuple()
        .expect("Invalid number of results");

    let mut coefficients = vec![vec![0.0; 2]; 2];
    let mut rhs = vec![0.0; 2];
    for i in 0..2 {
        let l1 = &hailstones[i];
        let l2 = &hailstones[i + 1];
        coefficients[i][0] = l1.velocity.0 - l2.velocity.0;
        coefficients[i][1] = l2.position.0 - l1.position.0;
        rhs[i] = -l1.position.0 * l1.velocity.2
            + l1.position.2 * l1.velocity.0
            + l2.position.0 * l2.velocity.2
            - l2.position.2 * l2.velocity.0
            - ((l2.velocity.2 - l1.velocity.2) * x)
            - ((l1.position.2 - l2.position.2) * vx);
    }

    let (z, vz) = gaussian_elimination(coefficients, rhs)
        .into_iter()
        .collect_tuple()
        .expect("Invalid number of results");

    let hailstone = Hailstone {
        position: (x, y, z),
        velocity: (vx, vy, vz),
    };

    println!(
        "{:?}",
        (hailstone.position.0 + hailstone.position.1 + hailstone.position.2).round() as isize
    );
}
