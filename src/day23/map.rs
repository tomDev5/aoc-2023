use itertools::Itertools;
use pathfinding::num_traits::ToPrimitive;

use crate::tile::Tile;

pub struct Map {
    inner: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        Self {
            inner: input
                .lines()
                .map(|line| {
                    line.chars()
                        .filter_map(|c| Tile::try_from(c).ok())
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    pub fn get(&self, (y, x): (isize, isize)) -> Option<&Tile> {
        self.inner.get(y.to_usize()?)?.get(x.to_usize()?)
    }

    pub fn get_first_position(&self) -> Option<(isize, isize)> {
        Some((
            0,
            self.inner
                .first()?
                .iter()
                .find_position(|tile| **tile == Tile::Path)?
                .0
                .to_isize()?,
        ))
    }

    pub fn get_last_position(&self) -> Option<(isize, isize)> {
        Some((
            (self.inner.len() - 1).to_isize()?,
            self.inner
                .last()?
                .iter()
                .find_position(|tile| **tile == Tile::Path)?
                .0 as isize,
        ))
    }
}
