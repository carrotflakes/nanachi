use std::marker::PhantomData;

use crate::{fill_color::FillColor, matrix::Matrix};

#[derive(Debug, Clone)]
pub struct Transform<'a, C: Clone, FC: FillColor<C>> {
    fill_color: &'a FC,
    matrix: Matrix,
    c: PhantomData<C>,
}

impl<'a, C: Clone, FC: FillColor<C>> Transform<'a, C, FC> {
    pub fn new(fill_color: &'a FC, matrix: Matrix) -> Self {
        Transform {
            fill_color,
            matrix: matrix.inverse(),
            c: PhantomData::default(),
        }
    }
}

impl<'a, C: Clone, FC: FillColor<C>> FillColor<C> for Transform<'a, C, FC> {
    fn fill_color(&self, pos: [f32; 2]) -> C {
        let p = self.matrix.apply(pos);
        self.fill_color.fill_color(p)
    }
}
