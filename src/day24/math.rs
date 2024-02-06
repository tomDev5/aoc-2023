pub fn get_line_equation_from_points(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
    let m = (p1.1 - p2.1) / (p1.0 - p2.0);
    let b = -p1.0 / (p2.0 - p1.0) * (p2.1 - p1.1) + p1.1;
    (m, b)
}

pub fn get_intersection_point((m1, b1): (f64, f64), (m2, b2): (f64, f64)) -> Option<(f64, f64)> {
    let (diff_m, diff_b) = (m1 - m2, b1 - b2);

    if diff_m == 0.0 {
        return None;
    }

    let x_intersection = -diff_b / diff_m;
    let y_intersection = m1 * x_intersection + b1;

    Some((x_intersection, y_intersection))
}

pub fn gaussian_elimination(mut coefficients: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Vec<f64> {
    for i in 0..coefficients.len() {
        // Select pivot
        let pivot = coefficients[i][i];
        // Normalize row i
        for j in 0..coefficients.len() {
            coefficients[i][j] /= pivot;
        }
        rhs[i] /= pivot;
        // Sweep using row i
        for k in 0..coefficients.len() {
            if k != i {
                let factor = coefficients[k][i];
                for j in 0..coefficients.len() {
                    coefficients[k][j] -= factor * coefficients[i][j];
                }
                rhs[k] -= factor * rhs[i];
            }
        }
    }
    rhs
}
