const INPUT: &'static str = include_str!("../../data/day01/input.txt");

fn main() {
    let sum: u32 = INPUT
        .lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next()?;
            let second_digit = digits.rev().next().unwrap_or(first_digit);
            Some(first_digit * 10 + second_digit)
        })
        .sum();
    println!("sum: {}", sum);
}
