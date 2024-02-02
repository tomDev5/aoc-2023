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

    pub fn get_next(&self, (y, x): (isize, isize)) -> impl Iterator<Item = (isize, isize)> + '_ {
        match self.get((y, x)).expect("Bad coordinates") {
            Tile::UpSlope => vec![(y - 1, x)],
            Tile::DownSlope => vec![(y + 1, x)],
            Tile::LeftSlope => vec![(y, x - 1)],
            Tile::RightSlope => vec![(y, x + 1)],
            Tile::Path => vec![(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
                .into_iter()
                .filter(|point| self.get(*point).is_some_and(|tile| *tile != Tile::Forest))
                .collect_vec(),
            Tile::Forest => unreachable!("Cannot pass through forest"),
        }
        .into_iter()
    }

    pub fn get_next_condensed(
        &self,
        mut previous: (isize, isize),
        mut point: (isize, isize),
        end: (isize, isize),
    ) -> impl Iterator<Item = ((isize, isize), usize)> + '_ {
        let mut cost = 1;

        loop {
            let next = self
                .get_next(point)
                .filter(|&p| p != previous)
                .filter(|&p| {
                    let next = self.get_next(p).collect_vec();
                    next.len() != 1 || next[0] != point || p == end
                })
                .collect_vec();

            if next.len() == 1 && next[0] != end {
                previous = point;
                point = next[0];
                cost += 1;
            } else {
                return next.into_iter().map(move |p| (p, cost));
            }
        }
    }
}
