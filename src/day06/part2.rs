use itertools::Itertools;

mod transpose;

const INPUT: &'static str = include_str!("../../data/day06/input.txt");

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    current_record: usize,
}

impl Race {
    fn new(time: usize, current_record: usize) -> Self {
        Self {
            time,
            current_record,
        }
    }
}

fn main() {
    let race = INPUT
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .skip(1)
                .fold(String::new(), |acc, s| acc + s)
                .parse::<usize>()
                .ok()
        })
        .collect_vec();
    let race = Race::new(race[0], race[1]);

    let result: usize = (0..race.time)
        .map(|hold_time| run_race(race.time, hold_time))
        .filter(|distance| distance > &race.current_record)
        .count();
    println!("{:?}", result);
}

fn run_race(time: usize, hold_time: usize) -> usize {
    if hold_time >= time {
        0
    } else {
        (time - hold_time) * hold_time
    }
}
