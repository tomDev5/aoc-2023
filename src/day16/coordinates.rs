use crate::direction::Direction;

#[derive(Eq, PartialEq, Debug, Default, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Coordinates(pub usize, pub usize);

impl Coordinates {
    pub fn new(line: usize, column: usize) -> Self {
        Coordinates(line, column)
    }

    pub fn move_to(&self, direction: &Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Coordinates(self.0.checked_sub(1)?, self.1),
            Direction::Down => Coordinates(self.0 + 1, self.1),
            Direction::Left => Coordinates(self.0, self.1.checked_sub(1)?),
            Direction::Right => Coordinates(self.0, self.1 + 1),
        })
    }
}
