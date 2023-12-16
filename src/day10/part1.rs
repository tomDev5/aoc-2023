use crate::maze::Maze;

mod maze;

const INPUT: &str = include_str!("../../data/day10/input.txt");

fn main() {
    let maze = Maze::new(INPUT);

    println!("{:?}", maze.get_loop().expect("Invalid input").len() / 2);
}
