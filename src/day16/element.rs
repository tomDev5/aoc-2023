use crate::direction::Direction;
use std::fmt::Display;

#[derive(Clone, Copy, Hash)]
pub enum Element {
    Air,
    Slash,
    BackSlash,
    Pipe,
    Dash,
}

impl TryFrom<char> for Element {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Element::Air),
            '/' => Ok(Element::Slash),
            '\\' => Ok(Element::BackSlash),
            '|' => Ok(Element::Pipe),
            '-' => Ok(Element::Dash),
            _ => Err(()),
        }
    }
}

impl Element {
    pub fn get_directions(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Element::Air => vec![direction],
            Element::Slash => match direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            Element::BackSlash => match direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            Element::Pipe => match direction {
                d @ (Direction::Up | Direction::Down) => vec![d],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
            Element::Dash => match direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                d @ (Direction::Left | Direction::Right) => vec![d],
            },
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Air => write!(f, "."),
            Element::Slash => write!(f, "/"),
            Element::BackSlash => write!(f, "\\"),
            Element::Pipe => write!(f, "|"),
            Element::Dash => write!(f, "-"),
        }
    }
}
