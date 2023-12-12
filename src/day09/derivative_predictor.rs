use itertools::Itertools;

pub fn get_prediction(line: impl Iterator<Item = isize>) -> Option<isize> {
    let derivatives = std::iter::successors(Some(line.collect::<Vec<_>>()), |last| {
        if last.iter().any(|&n| n != 0) {
            Some(last.iter().tuple_windows().map(|(n, m)| m - n).collect())
        } else {
            None
        }
    })
    .collect::<Vec<_>>();

    derivatives
        .iter()
        .rev()
        .fold(None, |line: Option<isize>, current| {
            Some(
                line.and_then(|line| Some(line + current.last()?))
                    .unwrap_or(0),
            )
        })
}
