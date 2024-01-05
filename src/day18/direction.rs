#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::U),
            'D' => Ok(Self::D),
            'L' => Ok(Self::L),
            'R' => Ok(Self::R),
            _ => Err(()),
        }
    }
}
