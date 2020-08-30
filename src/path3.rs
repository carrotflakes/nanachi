use crate::point::Point;
use crate::models::{Line, Arc, Ellipse, Quad};
use crate::geometry;

#[derive(Debug, Clone)]
pub enum PathItem {
    Line(Line),
    Arc(Arc),
    Ellipse(Ellipse),
    Quad(Quad),
}

impl PathItem {
    pub fn flip(&self) -> PathItem {
        match self {
            PathItem::Line(line) => PathItem::Line(Line(line.1, line.0)),
            PathItem::Arc(arc) => PathItem::Arc(Arc {
                center: arc.center,
                radius: arc.radius,
                angle1: arc.angle2,
                angle2: arc.angle1,
            }),
            PathItem::Ellipse(ellipse) => PathItem::Ellipse(Ellipse {
                center: ellipse.center,
                radius_x: ellipse.radius_x,
                radius_y: ellipse.radius_y,
                rotation: ellipse.rotation,
                angle1: ellipse.angle2,
                angle2: ellipse.angle1,
            }),
            PathItem::Quad(quad) => PathItem::Quad(Quad{
                start: quad.end.clone(),
                end: quad.start.clone(),
                control1: quad.control1.clone(),
            })
        }
    }

    pub fn right_point(&self) -> Point {
        match self {
            PathItem::Line(line) => line.1,
            PathItem::Arc(arc) => {
                arc.center
                    + Point(
                        arc.angle2.cos() * arc.radius,
                        arc.angle2.sin() * arc.radius,
                    )
            }
            PathItem::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle2.cos() * ellipse.radius_x;
                let y = ellipse.angle2.sin() * ellipse.radius_y;
                ellipse.center + Point(x * cos - y * sin, x * sin + y * cos)
            }
            PathItem::Quad(quad) => {
                quad.end
            }
        }
    }

    pub fn left_point(&self) -> Point {
        match self {
            PathItem::Line(line) => line.0,
            PathItem::Arc(arc) => {
                arc.center
                    + Point(
                        arc.angle1.cos() * arc.radius,
                        arc.angle1.sin() * arc.radius,
                    )
            }
            PathItem::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle1.cos() * ellipse.radius_x;
                let y = ellipse.angle1.sin() * ellipse.radius_y;
                ellipse.center + Point(x * cos - y * sin, x * sin + y * cos)
            }
            PathItem::Quad(quad) => {
                quad.start
            }
        }
    }

    pub fn intersect(&self, other: &PathItem) -> Option<Point> {
        match (self, other) {
            (PathItem::Line(Line(p1, p2)), PathItem::Line(Line(p3, p4))) =>
                crate::geometry::intersect_segment_and_segment(*p1, *p2, *p3, *p4),
            (PathItem::Line(line), PathItem::Arc(arc)) =>
                intersect_segment_and_arc(line, arc),
            (PathItem::Line(line), PathItem::Ellipse(ellipse)) =>
                intersect_segment_and_ellipse(line, ellipse),
            (PathItem::Line(line), PathItem::Quad(quad)) =>
                intersect_segment_and_quad(line, quad),
            (PathItem::Arc(arc), PathItem::Line(line)) =>
                intersect_segment_and_arc(line, arc),
            (PathItem::Arc(arc1), PathItem::Arc(arc2)) =>
                todo!(),
            (PathItem::Arc(arc), PathItem::Ellipse(ellipse)) =>
                todo!(),
            (PathItem::Arc(arc), PathItem::Quad(quad)) =>
                todo!(),
            (PathItem::Ellipse(ellipse), PathItem::Line(line)) =>
                intersect_segment_and_ellipse(line, ellipse),
            (PathItem::Ellipse(ellipse), PathItem::Arc(arc)) =>
                todo!(),
            (PathItem::Ellipse(ellipse1), PathItem::Ellipse(ellipse2)) =>
                intersect_ellipse_and_ellipse(ellipse1, ellipse2),
            (PathItem::Ellipse(ellipse), PathItem::Quad(quad)) =>
                intersect_ellipse_and_quad(ellipse, quad),
            (PathItem::Quad(quad), PathItem::Line(line)) =>
                intersect_segment_and_quad(line, quad),
            (PathItem::Quad(quad), PathItem::Arc(arc)) =>
                todo!(),
            (PathItem::Quad(quad), PathItem::Ellipse(ellipse)) =>
                intersect_ellipse_and_quad(ellipse, quad),
            (PathItem::Quad(quad1), PathItem::Quad(quad2)) =>
                intersect_quad_and_quad(quad1, quad2),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path(pub Vec<PathItem>);

impl Path {
    pub fn new(items: Vec<PathItem>) -> Path {
        Path(items)
    }

    pub fn is_closed(&self) -> bool {
        self.0[0].left_point() == self.0[self.0.len() - 1].right_point()
    }

    pub fn from_points(points: &Vec<Point>) -> Path {
        let mut pis = Vec::new();
        for i in 0..points.len() - 1 {
            pis.push(PathItem::Line(Line(
                points[i],
                points[i + 1],
            )));
        }
        Path(pis)
    }

    pub fn from_bezier2_points(points: &Vec<Point>) -> Path {
        let mut pis = Vec::new();
        for i in 0..points.len() / 2 {
            pis.push(PathItem::Quad(Quad {
                start: points[i * 2],
                end: points[i * 2 + 2],
                control1: points[i * 2 + 1],
            }));
        }
        Path(pis)
    }
}

pub fn intersect_segment_and_arc(line: &Line, arc: &Arc) -> Option<Point> {
    let p1 = line.0 - arc.center;
    let p2 = line.1 - arc.center;
    let d1 = p1.norm();
    let d2 = p2.norm();
    let p = if d1 <= arc.radius && arc.radius < d2 {
        geometry::intersect_circle_and_segment(line.0 / arc.radius, line.1 / arc.radius)
    } else if d2 <= arc.radius && arc.radius < d1 {
        geometry::intersect_circle_and_segment(line.1 / arc.radius, line.0 / arc.radius)
    } else {
        return None;
    };
    let angle = p.atan2().rem_euclid(2.0 * std::f64::consts::PI);
    let (a1, a2) = arc.angle_norm();
    if a1 <= angle && angle <= a2 {
        Some(p * arc.radius)
    } else {
        None
    }
}

pub fn intersect_segment_and_ellipse(line: &Line, ellipse: &Ellipse) -> Option<Point> {
    todo!()
}

pub fn intersect_segment_and_quad(line: &Line, quad: &Quad) -> Option<Point> {
    todo!()
}

pub fn intersect_ellipse_and_ellipse(e1: &Ellipse, e2: &Ellipse) -> Option<Point> {
    todo!()
}

pub fn intersect_ellipse_and_quad(ellipse: &Ellipse, quad: &Quad) -> Option<Point> {
    todo!()
}

pub fn intersect_quad_and_quad(q1: &Quad, q2: &Quad) -> Option<Point> {
    todo!()
}
