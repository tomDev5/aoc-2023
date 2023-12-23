use crate::heat_map::HeatMap;

mod heat_map;

const INPUT: &str = include_str!("../../data/day17/input.txt");

fn main() {
    let heat_map = HeatMap::new(
        INPUT
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10))),
    );

    println!("result: {:?}", heat_map.get_minimal_heat_loss::<3>());
}
