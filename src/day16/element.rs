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
    pub fn get_directions(&self, direction: Direction) -> impl Iterator<Item = Direction> + '_ {
        match self {
            Element::Air => vec![direction].into_iter(),
            Element::Slash => match direction {
                Direction::Up => vec![Direction::Right].into_iter(),
                Direction::Down => vec![Direction::Left].into_iter(),
                Direction::Left => vec![Direction::Down].into_iter(),
                Direction::Right => vec![Direction::Up].into_iter(),
            },
            Element::BackSlash => match direction {
                Direction::Up => vec![Direction::Left].into_iter(),
                Direction::Down => vec![Direction::Right].into_iter(),
                Direction::Left => vec![Direction::Up].into_iter(),
                Direction::Right => vec![Direction::Down].into_iter(),
            },
            Element::Pipe => match direction {
                d @ (Direction::Up | Direction::Down) => vec![d].into_iter(),
                Direction::Left | Direction::Right => {
                    vec![Direction::Up, Direction::Down].into_iter()
                }
            },
            Element::Dash => match direction {
                Direction::Up | Direction::Down => {
                    vec![Direction::Left, Direction::Right].into_iter()
                }
                d @ (Direction::Left | Direction::Right) => vec![d].into_iter(),
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
