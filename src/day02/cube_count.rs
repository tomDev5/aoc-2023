use itertools::Itertools;

#[derive(Default, Debug, Clone)]
pub struct CubeCount {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl TryFrom<&str> for CubeCount {
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

impl CubeCount {
    pub fn max_count(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}
