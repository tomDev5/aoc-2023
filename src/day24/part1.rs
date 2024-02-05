use hailstone::Hailstone;
use itertools::Itertools;

mod hailstone;

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

            let m = (first_point.1 - second_point.1) / (first_point.0 - second_point.0);
            let position_where_x_is_zero = -hailstone.position.0 / hailstone.velocity.0;
            let b = position_where_x_is_zero * hailstone.velocity.1 + hailstone.position.1;
            (m, b, hailstone.position.0, hailstone.velocity.0)
        })
        .combinations(2)
        .filter_map(|hailstones| hailstones.into_iter().collect_tuple())
        .filter_map(
            |((m1, b1, start_x1, x1_velocity), (m2, b2, start_x2, x2_velocity))| {
                let (diff_m, diff_b) = (m1 - m2, b1 - b2);

                if diff_m == 0.0 {
                    return None;
                }

                let x_intersection = -diff_b / diff_m;
                let y_intersection = m1 * x_intersection + b1;

                if (x_intersection < start_x1 && x1_velocity > 0.0)
                    || (x_intersection > start_x1 && x1_velocity < 0.0)
                {
                    return None;
                }

                if (x_intersection < start_x2 && x2_velocity > 0.0)
                    || (x_intersection > start_x2 && x2_velocity < 0.0)
                {
                    return None;
                }

                Some((x_intersection, y_intersection))
            },
        )
        .filter(|(x, y)| test_area.contains(x) && test_area.contains(y))
        .count();

    println!("{:?}", result);
}
