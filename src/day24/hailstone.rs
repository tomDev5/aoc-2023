use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Hailstone {
    pub position: (f64, f64, f64),
    pub velocity: (f64, f64, f64),
}

impl TryFrom<&str> for Hailstone {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (position, incline) = value
            .replace(',', "")
            .split('@')
            .filter_map(|part| {
                part.split_whitespace()
                    .filter_map(|s| s.parse::<f64>().ok())
                    .collect_tuple::<(_, _, _)>()
            })
            .collect_tuple()
            .ok_or(())?;
        Ok(Self {
            position,
            velocity: incline,
        })
    }
}
