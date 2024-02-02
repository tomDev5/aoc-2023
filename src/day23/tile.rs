use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Path,
    Forest,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Path => write!(f, "."),
            Tile::Forest => write!(f, "#"),
            Tile::UpSlope => write!(f, "^"),
            Tile::DownSlope => write!(f, "v"),
            Tile::LeftSlope => write!(f, "<"),
            Tile::RightSlope => write!(f, ">"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Path),
            '#' => Ok(Tile::Forest),
            '^' => Ok(Tile::UpSlope),
            'v' => Ok(Tile::DownSlope),
            '<' => Ok(Tile::LeftSlope),
            '>' => Ok(Tile::RightSlope),
            _ => Err(()),
        }
    }
}
