use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Garden,
    Rock,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Garden => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Element {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Garden),
            '#' => Ok(Self::Rock),
            _ => Err(()),
        }
    }
}
