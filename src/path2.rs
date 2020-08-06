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

#[derive(Debug, Clone)]
pub enum PathEdge {
    Line(Point, Point),
    Arc{
        center: Point,
        radius: f64,
        angle1: f64,
        angle2: f64,
    },
}

impl Path {
    pub fn new(anchors: Vec<PathAnchor>, close: bool) -> Path {
        Path { anchors, close }
    }

    pub fn edges(&self) -> Vec<PathEdge> {
        let mut edges = Vec::new();
        for i in 0..self.anchors.len() {
            match (&self.anchors[i], &self.anchors[(i + 1) % self.anchors.len()]) {
                (PathAnchor::Point(p1), PathAnchor::Point(p2)) => {
                    edges.push(PathEdge::Line(*p1, *p2));
                }
                (PathAnchor::Point(p), PathAnchor::Arc { center, radius, angle1, angle2 }) => {
                    let (sin, cos) = angle1.sin_cos();
                    edges.push(PathEdge::Line(*p, *center + Point(cos * radius, -sin * radius)));
                    edges.push(PathEdge::Arc {center: *center, radius: *radius, angle1: *angle1, angle2: *angle2});
                }
                (PathAnchor::Arc { center, radius, angle1: _, angle2 }, PathAnchor::Point(p)) => {
                    let (sin, cos) = angle2.sin_cos();
                    edges.push(PathEdge::Line(*center + Point(cos * radius, -sin * radius), *p));
                }
                (PathAnchor::Arc { center: c1, radius: r1, angle1: _, angle2: a12 }, PathAnchor::Arc { center: c2, radius: r2, angle1: a21, angle2: a22 }) => {
                    let (sin1, cos1) = a12.sin_cos();
                    let (sin2, cos2) = a21.sin_cos();
                    edges.push(PathEdge::Line(*c1 + Point(cos1 * r1, -sin1 * r1), *c2 + Point(cos2 * r2, -sin2 * r2)));
                    edges.push(PathEdge::Arc {center: *c2, radius: *r2, angle1: *a21, angle2: *a22});
                }
            }
        }
        edges
    }
}
