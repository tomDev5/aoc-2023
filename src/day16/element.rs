use arrayvec::ArrayVec;

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
    pub fn get_directions(&self, direction: Direction) -> ArrayVec<Direction, 2> {
        match self {
            Element::Air => ArrayVec::from_iter(std::iter::once(direction)),
            Element::Slash => match direction {
                Direction::Up => ArrayVec::from_iter(std::iter::once(Direction::Right)),
                Direction::Down => ArrayVec::from_iter(std::iter::once(Direction::Left)),
                Direction::Left => ArrayVec::from_iter(std::iter::once(Direction::Down)),
                Direction::Right => ArrayVec::from_iter(std::iter::once(Direction::Up)),
            },
            Element::BackSlash => match direction {
                Direction::Up => ArrayVec::from_iter(std::iter::once(Direction::Left)),
                Direction::Down => ArrayVec::from_iter(std::iter::once(Direction::Right)),
                Direction::Left => ArrayVec::from_iter(std::iter::once(Direction::Up)),
                Direction::Right => ArrayVec::from_iter(std::iter::once(Direction::Down)),
            },
            Element::Pipe => match direction {
                d @ (Direction::Up | Direction::Down) => ArrayVec::from_iter(std::iter::once(d)),
                Direction::Left | Direction::Right => {
                    ArrayVec::from_iter([Direction::Up, Direction::Down].into_iter())
                }
            },
            Element::Dash => match direction {
                Direction::Up | Direction::Down => {
                    ArrayVec::from_iter([Direction::Left, Direction::Right].into_iter())
                }
                d @ (Direction::Left | Direction::Right) => ArrayVec::from_iter(std::iter::once(d)),
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
