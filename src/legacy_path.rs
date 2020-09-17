use crate::matrix::Matrix2d;
use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn is_closed(&self) -> bool {
        if self.points.is_empty() {
            true
        } else {
            &self.points[0] == self.points.last().unwrap()
        }
    }

    pub fn transform_mut(&mut self, am: &Matrix2d) {
        for p in self.points.as_mut_slice() {
            *p = am.apply(*p);
        }
    }
}

impl From<Vec<Point>> for Path {
    fn from(points: Vec<Point>) -> Path {
        Path { points }
    }
}

impl Into<Vec<Point>> for Path {
    fn into(self) -> Vec<Point> {
        self.points
    }
}
