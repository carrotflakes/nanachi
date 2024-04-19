//! [`Path`] for filling and stroking.

use crate::models::{Arc, Cubic, Ellipse, Line, Quad};
use crate::point::Point;

#[derive(Debug, Clone)]
pub enum PathItem {
    Line(Line),
    Arc(Arc),
    Ellipse(Ellipse),
    Quad(Quad),
    Cubic(Cubic),
    CloseAndJump,
    Jump,
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
            PathItem::Quad(quad) => PathItem::Quad(Quad {
                start: quad.end.clone(),
                end: quad.start.clone(),
                control1: quad.control1.clone(),
            }),
            PathItem::Cubic(cubic) => PathItem::Cubic(Cubic {
                start: cubic.end.clone(),
                end: cubic.start.clone(),
                control1: cubic.control2.clone(),
                control2: cubic.control1.clone(),
            }),
            PathItem::CloseAndJump => unreachable!(),
            PathItem::Jump => unreachable!(),
        }
    }

    pub fn right_point(&self) -> Point {
        match self {
            PathItem::Line(line) => line.1,
            PathItem::Arc(arc) => {
                arc.center
                    + Point::from((arc.angle2.cos() * arc.radius, arc.angle2.sin() * arc.radius))
            }
            PathItem::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle2.cos() * ellipse.radius_x;
                let y = ellipse.angle2.sin() * ellipse.radius_y;
                ellipse.center + Point::from((x * cos - y * sin, x * sin + y * cos))
            }
            PathItem::Quad(quad) => quad.end,
            PathItem::Cubic(cubic) => cubic.end,
            PathItem::CloseAndJump => unreachable!(),
            PathItem::Jump => unreachable!(),
        }
    }

    pub fn left_point(&self) -> Point {
        match self {
            PathItem::Line(line) => line.0,
            PathItem::Arc(arc) => {
                arc.center
                    + Point::from((arc.angle1.cos() * arc.radius, arc.angle1.sin() * arc.radius))
            }
            PathItem::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle1.cos() * ellipse.radius_x;
                let y = ellipse.angle1.sin() * ellipse.radius_y;
                ellipse.center + Point::from((x * cos - y * sin, x * sin + y * cos))
            }
            PathItem::Quad(quad) => quad.start,
            PathItem::Cubic(cubic) => cubic.start,
            PathItem::CloseAndJump => unreachable!(),
            PathItem::Jump => unreachable!(),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            PathItem::Line(line) => line.0 == line.1,
            PathItem::Arc(arc) => arc.radius == 0.0 || arc.angle1 == arc.angle2,
            PathItem::Ellipse(ellipse) => {
                (ellipse.radius_x == 0.0 && ellipse.radius_y == 0.0)
                    || ellipse.angle1 == ellipse.angle2
            }
            PathItem::Quad(quad) => quad.start == quad.end && quad.start == quad.control1,
            PathItem::Cubic(cubic) => {
                cubic.start == cubic.end
                    && cubic.start == cubic.control1
                    && cubic.start == cubic.control2
            }
            PathItem::CloseAndJump => false,
            PathItem::Jump => false,
        }
    }

    pub fn is_jump(&self) -> bool {
        match self {
            PathItem::CloseAndJump | PathItem::Jump => true,
            _ => false,
        }
    }
}

/// Path for filling and stroking.
///
/// A path can contains lines, ellipse arcs, quadratic bezier curves and cubic bezier curves.
/// You should use [`PathBuilder`] for creating a path.
#[derive(Debug, Clone)]
pub struct Path(pub Vec<PathItem>);

impl Path {
    /// Create [`Path`].
    pub fn new(items: Vec<PathItem>) -> Path {
        Path(items)
    }

    /// Merge 2 paths.
    pub fn merge(&mut self, rhs: &Path) {
        if !self.0.is_empty() && !self.0.last().unwrap().is_jump() {
            self.0.push(PathItem::Jump);
        }
        self.0.extend_from_slice(rhs.0.as_slice());
    }

    /// Create [`Path`] from `Vec<Point>`.
    pub fn from_points(points: &Vec<Point>, close: bool) -> Path {
        let mut pis = Vec::new();
        for i in 0..points.len() - 1 {
            pis.push(PathItem::Line(Line(points[i], points[i + 1])));
        }
        if close && &points[0] == points.last().unwrap() {
            pis.push(PathItem::CloseAndJump);
        }
        Path(pis)
    }

    /// Create [`Path`] as a bezire curves from `Vec<Point>`.
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

    /// Get [`Point`]s lists from a [`Path`] composed only lines.
    pub fn as_points_list(&self) -> Option<Vec<Vec<Point>>> {
        let mut vec = Vec::new();
        let mut points = Vec::new();
        let mut first_line = true;
        for pi in self.0.iter() {
            match pi {
                PathItem::Line(l) => {
                    if first_line {
                        points.push(l.0);
                        first_line = false;
                    }
                    points.push(l.1);
                }
                PathItem::Arc(_)
                | PathItem::Ellipse(_)
                | PathItem::Quad(_)
                | PathItem::Cubic(_) => {
                    return None;
                }
                PathItem::CloseAndJump | PathItem::Jump => {
                    vec.push(points);
                    points = Vec::new();
                    first_line = true
                }
            }
        }
        if !points.is_empty() {
            vec.push(points);
        }
        Some(vec)
    }

    /// Get continuous [`PathItem`]s in the path.
    pub fn continuations<'a>(&'a self) -> Vec<(&'a [PathItem], bool)> {
        let mut pis = self.0.as_slice();
        let mut i = 0;
        let mut res = Vec::new();
        while let Some(pi) = pis.get(i) {
            match pi {
                PathItem::CloseAndJump => {
                    let (left, right) = pis.split_at(i);
                    res.push((left, true));
                    pis = &right[1..];
                    i = 0;
                }
                PathItem::Jump => {
                    let (left, right) = pis.split_at(i);
                    res.push((left, false));
                    pis = &right[1..];
                    i = 0;
                }
                _ => {
                    i += 1;
                }
            }
        }
        if 1 <= pis.len() {
            res.push((pis, false));
        }
        res
    }

    /// Flip path direction.
    pub fn flip(&self) -> Path {
        Path(
            self.continuations()
                .into_iter()
                .flat_map(|(pis, closed)| {
                    pis.iter().rev().map(|pi| pi.flip()).chain(vec![if closed {
                        PathItem::CloseAndJump
                    } else {
                        PathItem::Jump
                    }])
                })
                .collect(),
        )
    }
}
