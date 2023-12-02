const INPUT: &'static str = include_str!("../../data/day02/input.txt");

mod cube_count;

use cube_count::CubeCount;

fn main() {
    let sum: usize = INPUT
        .lines()
        .filter_map(|line| line.split(":").nth(1))
        .map(|line| line.split(";"))
        .map(|game| game.filter_map(|drawing_str| CubeCount::try_from(drawing_str).ok()))
        .map(|game| game.fold(CubeCount::default(), CubeCount::max_count))
        .enumerate()
        .filter(|(_, drawing)| drawing.red <= 12 && drawing.green <= 13 && drawing.blue <= 14)
        .map(|(index, _)| index + 1)
        .sum();
    assert_eq!(sum, 2061);
}
