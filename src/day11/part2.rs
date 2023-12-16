use crate::universe::Universe;

mod cosmic_element;
mod transpose;
mod universe;

const INPUT: &str = include_str!("../../data/day11/input.txt");

fn main() {
    let universe = Universe::new(INPUT);
    let distances = universe
        .get_galaxy_distances(999_999)
        .into_iter()
        .map(|(_, _, d)| d)
        .sum::<usize>()
        / 2;

    println!("result: {:?}", distances);
}
