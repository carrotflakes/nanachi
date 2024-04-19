//! PathBuilder

use crate::{
    models::{Arc, Cubic, Ellipse, Line, Quad},
    path::{Path, PathItem},
    point::Point,
};
use std::f32::consts::TAU;

/// Path builder.
#[derive(Debug, Clone)]
pub struct PathBuilder {
    items: Vec<PathItem>,
    last_pos: Option<Point>,
    path_start: Option<Point>,
}

impl PathBuilder {
    /// Create [`PathBuilder`].
    pub fn new() -> PathBuilder {
        PathBuilder {
            items: vec![],
            last_pos: None,
            path_start: None,
        }
    }

    fn push(&mut self, pi: PathItem) {
        if !pi.is_zero() {
            self.items.push(pi);
        }
    }

    fn set_pos(&mut self, p: Point) {
        if self.path_start == None {
            self.path_start = Some(p);
        }
        self.last_pos = Some(p);
    }

    /// Set current position.
    pub fn move_to(&mut self, x: f32, y: f32) {
        match self.items.last() {
            Some(PathItem::CloseAndJump) | Some(PathItem::Jump) | None => {}
            Some(_) => {
                self.push(PathItem::Jump);
                self.path_start = None;
            }
        }
        self.set_pos(Point::from((x, y)));
    }

    /// Add a segment that from current position to specified position. And then set the end position to current position.
    pub fn line_to(&mut self, x: f32, y: f32) {
        let p = Point::from((x, y));
        if let Some(last_pos) = self.last_pos {
            self.push(PathItem::Line(Line([last_pos, p])));
        }
        self.set_pos(p);
    }

    /// Add an arc.
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, angle1: f32, angle2: f32) {
        let center = Point::from((x, y));
        let arc = PathItem::Arc(Arc {
            center,
            radius,
            angle1,
            angle2,
        });
        if let Some(last_pos) = self.last_pos {
            let left_point = arc.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line([last_pos, left_point])));
            }
        }
        self.set_pos(arc.right_point());
        self.push(arc);
    }

    /// Add an ellipse.
    pub fn ellipse(
        &mut self,
        x: f32,
        y: f32,
        radius_x: f32,
        radius_y: f32,
        rotation: f32,
        angle1: f32,
        angle2: f32,
    ) {
        let radius_x = radius_x.abs();
        let radius_y = radius_y.abs();
        let center = Point::from((x, y));
        let ellipse = PathItem::Ellipse(Ellipse {
            center,
            radius_x,
            radius_y,
            rotation,
            angle1,
            angle2,
        });
        if let Some(last_pos) = self.last_pos {
            let left_point = ellipse.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line([last_pos, left_point])));
            }
        }
        self.set_pos(ellipse.right_point());
        self.push(ellipse);
    }

    /// Add an endpoint-parameterized ellipse.
    pub fn ellipse_from_endpoint(
        &mut self,
        radius_x: f32,
        radius_y: f32,
        rotation: f32,
        large: bool,
        clockwise: bool,
        x: f32,
        y: f32,
    ) {
        let start = self.last_pos.unwrap_or_else(|| {
            panic!("PathBuilder::move_to() is required before ellipse_from_endpoint")
        });
        let end = Point::from((x, y));
        if radius_x == 0.0 || radius_y == 0.0 {
            self.set_pos(end);
            self.push(PathItem::Line(Line([start, end])));
            return;
        }
        let mut radius_x = radius_x.abs();
        let mut radius_y = radius_y.abs();
        let p = (start - end).rotate(-rotation) / 2.0;
        {
            let s = (p.x() / radius_x).powi(2) + (p.y() / radius_y).powi(2);
            if 1.0 < s {
                radius_x *= s.sqrt();
                radius_y *= s.sqrt();
            }
        }
        let (rx2, ry2) = (radius_x.powi(2), radius_y.powi(2));
        let mut a = ((rx2 * ry2 - rx2 * p.y().powi(2) - ry2 * p.x().powi(2))
            / (rx2 * p.y().powi(2) + ry2 * p.x().powi(2)))
        .sqrt();
        if large == clockwise {
            a = -a;
        }
        let q = Point::from((radius_x * p.y() / radius_y, -radius_y * p.x() / radius_x)) * a;
        let center = q.rotate(rotation) + (start + end) / 2.0;
        let a1 = Point::from(((p.x() - q.x()) / radius_x, (p.y() - q.y()) / radius_y));
        let mut angle1 = (a1.x() / a1.norm()).acos().copysign(a1.y());
        let a2 = Point::from((-(p.x() + q.x()) / radius_x, -(p.y() + q.y()) / radius_y));
        let mut angle2 = (a2.x() / a2.norm()).acos().copysign(a2.y());
        if clockwise && angle2 < angle1 {
            angle2 += TAU;
        }
        if !clockwise && angle1 < angle2 {
            angle1 += TAU;
        }
        let ellipse = PathItem::Ellipse(Ellipse {
            center,
            radius_x,
            radius_y,
            rotation,
            angle1,
            angle2,
        });
        self.set_pos(end);
        self.push(ellipse);
    }

    /// Add a quadratic bezier curve.
    pub fn quad(&mut self, control_x: f32, control_y: f32, x: f32, y: f32) {
        if let Some(last_pos) = self.last_pos {
            let quad = PathItem::Quad(Quad {
                start: last_pos,
                end: Point::from((x, y)),
                control1: Point::from((control_x, control_y)),
            });
            let left_point = quad.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line([last_pos, left_point])));
            }
            self.set_pos(quad.right_point());
            self.push(quad);
        } else {
            panic!("PathBuilder::move_to() is required before quad");
        }
    }

    /// Add a cubic bezier curve.
    pub fn cubic(
        &mut self,
        control_x1: f32,
        control_y1: f32,
        control_x2: f32,
        control_y2: f32,
        x: f32,
        y: f32,
    ) {
        if let Some(last_pos) = self.last_pos {
            let cubic = PathItem::Cubic(Cubic {
                start: last_pos,
                end: Point::from((x, y)),
                control1: Point::from((control_x1, control_y1)),
                control2: Point::from((control_x2, control_y2)),
            });
            let left_point = cubic.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line([last_pos, left_point])));
            }
            self.set_pos(cubic.right_point());
            self.push(cubic);
        } else {
            panic!("PathBuilder::move_to() is required before cubic");
        }
    }

    /// Close the path.
    pub fn close(&mut self) {
        if let Some(p) = self.path_start {
            self.line_to(p.x(), p.y());
            self.push(PathItem::CloseAndJump);
            self.path_start = None;
            self.last_pos = None;
        }
    }

    /// Return the built Path.
    pub fn end(&mut self) -> Path {
        let mut items = Vec::new();
        std::mem::swap(&mut items, &mut self.items);
        Path(items)
    }

    /// Return current position.
    pub fn current_pos(&self) -> Option<(f32, f32)> {
        self.last_pos.map(|p| p.into())
    }
}
