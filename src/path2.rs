use crate::point::Point;

#[derive(Debug, Clone)]
pub enum PathAnchor {
    Point(Point),
    Arc{
        center: Point,
        radius: f64,
        angle1: f64,
        angle2: f64,
    },
}

#[derive(Debug, Clone)]
pub struct Path {
    pub anchors: Vec<PathAnchor>,
    pub close: bool,
}

impl Path {
    pub fn new(anchors: Vec<PathAnchor>, close: bool) -> Path {
        Path { anchors, close }
    }
}
