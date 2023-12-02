const DIGIT_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const INPUT: &'static str = include_str!("../../data/day01/input.txt");

fn main() {
    let sum: u32 = INPUT
        .lines()
        .map(|line| line.to_string())
        .map(|line| {
            DIGIT_NAMES
                .into_iter()
                .enumerate()
                .fold(line, |line, (digit, digit_name)| {
                    line.replace(digit_name, &format!("{digit_name}{digit}{digit_name}"))
                })
        })
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next()?;
            let second_digit = digits.next_back().unwrap_or(first_digit);
            Some(first_digit * 10 + second_digit)
        })
        .sum();
    println!("sum: {}", sum);
}
