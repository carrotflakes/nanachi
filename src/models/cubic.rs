use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Cubic {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
    pub control2: Point,
}
