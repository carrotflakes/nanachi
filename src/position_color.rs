use crate::point::Point;

pub trait PositionColor<C> {
    fn position_color(&self, p: Point) -> C;
}

#[derive(Debug, Clone)]
pub struct Constant<C: Clone>(C);

impl<C: Clone> Constant<C> {
    pub fn new(c: C) -> Self {
        Constant(c)
    }
}

impl<C: Clone> PositionColor<C> for Constant<C> {
    fn position_color(&self, _: Point) -> C {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BlockCheck<C: Clone>(C, C, f64);

impl<C: Clone> BlockCheck<C> {
    pub fn new(c1: C, c2: C, size: f64) -> Self {
        BlockCheck(c1, c2, size)
    }
}

impl<C: Clone> PositionColor<C> for BlockCheck<C> {
    fn position_color(&self, p: Point) -> C {
        if ((p.0 / self.2) as isize + (p.1 / self.2) as isize) % 2 == 0 {
            self.0.clone()
        } else {
            self.1.clone()
        }
    }
}
