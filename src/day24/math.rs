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
