use std::marker::PhantomData;

use crate::{fill_color::FillColor, matrix::Matrix2d, point::Point};

#[derive(Debug, Clone)]
pub struct Transform<C: Clone, FC: FillColor<C>> {
    fill_color: FC,
    matrix: Matrix2d,
    c: PhantomData<C>,
}

impl<C: Clone, FC: FillColor<C>> Transform<C, FC> {
    pub fn new(fill_color: FC, matrix: Matrix2d) -> Self {
        Transform {
            fill_color,
            matrix: matrix.inverse(),
            c: PhantomData::default(),
        }
    }
}

impl<C: Clone, FC: FillColor<C>> FillColor<C> for Transform<C, FC> {
    fn fill_color(&self, x: f64, y: f64) -> C {
        let p = self.matrix.apply(Point(x, y));
        self.fill_color.fill_color(p.0, p.1)
    }
}
