use itertools::Itertools;
use maze::Maze;

mod maze;

const INPUT: &str = include_str!("../../data/day10/input.txt");

fn main() {
    let mut loop_coordinates = Maze::new(INPUT)
        .get_loop()
        .expect("Invalid input")
        .into_iter()
        .map(|(line, col)| (line as isize, col as isize))
        .collect_vec();
    loop_coordinates.push(loop_coordinates[0]);

    let loop_size = loop_coordinates.len() as isize;
    let loop_area_with_pipes = loop_coordinates
        .into_iter()
        .tuple_windows()
        .map(|((y0, x0), (y1, x1))| x0 * y1 - y0 * x1)
        .sum::<isize>()
        .abs()
        / 2;
    let enclosed_area = loop_area_with_pipes - loop_size / 2 + 1;
    println!("{}", enclosed_area);
}
