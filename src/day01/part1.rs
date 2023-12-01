const INPUT: &'static str = include_str!("../../data/day01/input.txt");

fn main() {
    let sum: u32 = INPUT
        .lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_number = digits.next()?;
            let second_number = digits.rev().next().unwrap_or(first_number);
            Some(first_number * 10 + second_number)
        })
        .sum();
    println!("sum: {}", sum);
}
