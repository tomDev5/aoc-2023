use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct PointAndPrevious {
    pub previous: (isize, isize),
    pub point: (isize, isize),
}

impl PointAndPrevious {
    pub fn new(previous: (isize, isize), point: (isize, isize)) -> Self {
        Self { previous, point }
    }
}

impl From<((isize, isize), (isize, isize))> for PointAndPrevious {
    fn from((previous, point): ((isize, isize), (isize, isize))) -> Self {
        Self::new(previous, point)
    }
}

impl PartialEq for PointAndPrevious {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
impl Eq for PointAndPrevious {}

impl Hash for PointAndPrevious {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}
