pub fn picks(corners: &Vec<(isize, isize)>) -> isize {
    let outline_length = corners
        .iter()
        .fold((0, (0, 0)), |(length, previous), point| {
            (
                length
                    + isize::abs_diff(previous.0, point.0)
                    + isize::abs_diff(previous.1, point.1),
                *point,
            )
        })
        .0 as isize;
    shoelace(corners) - (outline_length / 2) + 1 + outline_length
}

pub fn shoelace(corners: &Vec<(isize, isize)>) -> isize {
    let mut area = 0isize;

    for i in 0..corners.len() {
        let j = (i + 1) % corners.len();
        area += corners[i].0 * corners[j].1;
        area -= corners[j].0 * corners[i].1;
    }

    area.abs() / 2
}
