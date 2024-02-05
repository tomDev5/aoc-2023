use hailstone::Hailstone;
use itertools::Itertools;

use crate::{math::get_intersection_point, math::get_line_equation_from_points};

mod hailstone;
mod math;

const INPUT: &str = include_str!("../../data/day24/input.txt");

fn main() {
    let test_area = 200000000000000.0..=400000000000000.0;
    let result = INPUT
        .lines()
        .filter_map(|line| Hailstone::try_from(line).ok())
        .map(|hailstone| {
            let first_point = (hailstone.position.0, hailstone.position.1);
            let second_point = (
                hailstone.position.0 + hailstone.velocity.0,
                hailstone.position.1 + hailstone.velocity.1,
            );

            let (m, b) = get_line_equation_from_points(first_point, second_point);
            (m, b, hailstone)
        })
        .combinations(2)
        .filter_map(|hailstones| hailstones.into_iter().collect_tuple())
        .filter_map(|((m1, b1, hailstone1), (m2, b2, hailstone2))| {
            let (x_intersection, y_intersection) = get_intersection_point((m1, b1), (m2, b2))?;

            if (x_intersection < hailstone1.position.0 && hailstone1.velocity.0 > 0.0)
                || (x_intersection > hailstone1.position.0 && hailstone1.velocity.0 < 0.0)
                || (x_intersection < hailstone2.position.0 && hailstone2.velocity.0 > 0.0)
                || (x_intersection > hailstone2.position.0 && hailstone2.velocity.0 < 0.0)
            {
                return None;
            }

            Some((x_intersection, y_intersection))
        })
        .filter(|(x, y)| test_area.contains(x) && test_area.contains(y))
        .count();

    println!("{:?}", result);
}
