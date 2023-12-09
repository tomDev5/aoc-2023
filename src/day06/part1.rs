use itertools::Itertools;
use transpose::Transpose;

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
    let races = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec()
        })
        .collect_vec()
        .transpose()
        .into_iter()
        .map(|v| Race::new(v[0], v[1]));

    let result: usize = races
        .map(|race| {
            (0..race.time)
                .map(|hold_time| run_race(race.time, hold_time))
                .filter(|distance| distance > &race.current_record)
                .count()
        })
        .product();
    println!("{:?}", result);
}

fn run_race(time: usize, hold_time: usize) -> usize {
    if hold_time >= time {
        0
    } else {
        (time - hold_time) * hold_time
    }
}
