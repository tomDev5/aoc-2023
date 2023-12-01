const NUMBER_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const INPUT: &'static str = include_str!("../../data/day01/input.txt");

fn main() {
    let sum: u32 = INPUT
        .lines()
        .map(|line| line.to_string())
        .filter_map(|line| {
            NUMBER_NAMES
                .into_iter()
                .enumerate()
                .try_fold(line, |line, (number, number_string)| {
                    let first_char = number_string.chars().next()?;
                    let last_char = number_string.chars().next_back()?;
                    let target = format!("{first_char}{number}{last_char}");
                    Some(line.replace(number_string, &target))
                })
        })
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_number = digits.next()?;
            let second_number = digits.next_back().unwrap_or(first_number);
            Some(first_number * 10 + second_number)
        })
        .sum();
    println!("sum: {}", sum);
}
