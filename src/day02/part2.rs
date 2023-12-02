use itertools::Itertools;

const INPUT: &'static str = include_str!("../../data/day02/input.txt");

#[derive(Default, Debug)]
struct Drawing {
    red: usize,
    green: usize,
    blue: usize,
}

impl TryFrom<&str> for Drawing {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.split(",")
            .try_fold(Self::default(), |mut drawing, color_line| {
                let (number_of_cubes, color) = color_line.split_whitespace().collect_tuple()?;
                let number_of_cubes = number_of_cubes.parse::<usize>().ok()?;
                match color {
                    "red" => drawing.red += number_of_cubes,
                    "green" => drawing.green += number_of_cubes,
                    "blue" => drawing.blue += number_of_cubes,
                    _ => return None,
                }
                Some(drawing)
            })
            .ok_or(())
    }
}

fn main() {
    let sum: usize = INPUT
        .lines()
        .filter_map(|line| line.split(":").nth(1))
        .map(|line| line.split(";"))
        .map(|game| game.filter_map(|drawing_str| Drawing::try_from(drawing_str).ok()))
        .map(|game| {
            game.fold(Drawing::default(), |mut maxes, current| {
                maxes.red = maxes.red.max(current.red);
                maxes.green = maxes.green.max(current.green);
                maxes.blue = maxes.blue.max(current.blue);
                maxes
            })
        })
        .map(|drawing| drawing.red * drawing.green * drawing.blue)
        .sum();
    println!("sum: {}", sum);
}
