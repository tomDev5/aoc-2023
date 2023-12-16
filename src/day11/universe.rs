use crate::{cosmic_element::CosmicElement, transpose::Transpose};
use itertools::Itertools;

pub struct Universe {
    grid: Vec<Vec<CosmicElement>>,
}

impl Universe {
    pub fn new(input: &str) -> Universe {
        Universe {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .filter_map(|c| CosmicElement::try_from(c).ok())
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    pub fn get_galaxy_distances(
        &self,
        expantion: usize,
    ) -> Vec<((usize, usize), (usize, usize), usize)> {
        let empty_lines = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        let galaxies = self.get_galaxys();

        galaxies
            .clone()
            .into_iter()
            .cartesian_product(galaxies)
            .into_iter()
            .map(move |(p0, p1)| {
                let empty_lines_crossed = empty_lines
                    .iter()
                    .cloned()
                    .filter(|l| *l >= usize::min(p0.0, p1.0) && *l <= usize::max(p0.0, p1.0))
                    .count();
                let empty_cols_crossed = empty_cols
                    .iter()
                    .cloned()
                    .filter(|l| *l >= usize::min(p0.1, p1.1) && *l <= usize::max(p0.1, p1.1))
                    .count();

                (
                    p0,
                    p1,
                    distance(
                        p0,
                        p1,
                        (empty_lines_crossed + empty_cols_crossed) * expantion,
                    ),
                )
            })
            .collect()
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, l)| l.iter().all(|x| *x == CosmicElement::Void))
            .map(|(index, _)| index)
            .collect()
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        self.grid
            .clone()
            .transpose()
            .iter()
            .enumerate()
            .filter(|(_, l)| l.iter().all(|x| *x == CosmicElement::Void))
            .map(|(index, _)| index)
            .collect()
    }

    fn get_galaxys(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(column_index, element)| {
                        if *element == CosmicElement::Galaxy {
                            Some((line_index, column_index))
                        } else {
                            None
                        }
                    })
            })
            .flatten()
            .collect()
    }
}

fn distance(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    empty_rows_or_cols: usize,
) -> usize {
    usize::abs_diff(x1, x2) + usize::abs_diff(y1, y2) + empty_rows_or_cols
}
