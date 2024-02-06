use hailstone::Hailstone;
use itertools::Itertools;

mod hailstone;
mod math;

const INPUT: &str = include_str!("../../data/day24/input.txt");

fn main() {
    let hailstones = INPUT
        .lines()
        .filter_map(|line| Hailstone::try_from(line).ok())
        .collect_vec();

    let mut coefficients = vec![vec![0.0; 4]; 4];
    let mut rhs = [0.0; 4];

    // Get X,Y,VX,VY
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

    gaussian_elimination(&mut coefficients, &mut rhs);

    let x = rhs[0];
    let y = rhs[1];
    let vx = rhs[2];
    let vy = rhs[3];

    // Get X,VZ
    coefficients = vec![vec![0.0; 2]; 2];
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
            - ((l2.velocity.2 - l1.velocity.2) * x as f64)
            - ((l1.position.2 - l2.position.2) * vx as f64);
    }

    gaussian_elimination(&mut coefficients, &mut rhs);

    let z = rhs[0];
    let vz = rhs[1];

    let hailstone = Hailstone {
        position: (x, y, z),
        velocity: (vx, vy, vz),
    };

    println!(
        "{:?}",
        (hailstone.position.0 + hailstone.position.1 + hailstone.position.2).round() as isize
    );

    // 828418331313365 worked
}

fn gaussian_elimination(coefficients: &mut Vec<Vec<f64>>, rhs: &mut [f64]) {
    let nr_variables = coefficients.len();
    for i in 0..nr_variables {
        // Select pivot
        let pivot = coefficients[i][i];
        // Normalize row i
        for j in 0..nr_variables {
            coefficients[i][j] /= pivot;
        }
        rhs[i] /= pivot;
        // Sweep using row i
        for k in 0..nr_variables {
            if k != i {
                let factor = coefficients[k][i];
                for j in 0..nr_variables {
                    coefficients[k][j] -= factor * coefficients[i][j];
                }
                rhs[k] -= factor * rhs[i];
            }
        }
    }
}
