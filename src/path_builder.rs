use crate::{
    models::{Arc, Cubic, Ellipse, Line, Quad},
    path::{Path, PathItem},
    point::Point,
};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct PathBuilder {
    items: Vec<PathItem>,
    last_pos: Option<Point>,
    path_start: Option<Point>,
}

impl PathBuilder {
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

    pub fn move_to(&mut self, x: f64, y: f64) {
        match self.items.last() {
            Some(PathItem::CloseAndJump) | Some(PathItem::Jump) | None => {}
            Some(_) => {
                self.push(PathItem::Jump);
            }
        }
        self.set_pos(Point(x, y));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        let p = Point(x, y);
        if let Some(last_pos) = self.last_pos {
            self.push(PathItem::Line(Line(last_pos, p)));
        }
        self.set_pos(p);
    }

    pub fn arc(&mut self, x: f64, y: f64, radius: f64, angle1: f64, angle2: f64) {
        let center = Point(x, y);
        let arc = PathItem::Arc(Arc {
            center,
            radius,
            angle1,
            angle2,
        });
        if let Some(last_pos) = self.last_pos {
            let left_point = arc.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
        }
        self.set_pos(arc.right_point());
        self.push(arc);
    }

    pub fn ellipse(
        &mut self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        angle1: f64,
        angle2: f64,
    ) {
        let radius_x = radius_x.abs();
        let radius_y = radius_y.abs();
        let center = Point(x, y);
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
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
        }
        self.set_pos(ellipse.right_point());
        self.push(ellipse);
    }

    pub fn ellipse_from_endpoint(
        &mut self,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        large: bool,
        clockwise: bool,
        x: f64,
        y: f64,
    ) {
        let start = self.last_pos.unwrap_or_else(|| panic!("PathBuilder::start() is required"));
        let end = Point(x, y);
        if radius_x == 0.0 || radius_y == 0.0 {
            self.set_pos(end);
            self.push(PathItem::Line(Line(start, end)));
            return;
        }
        let mut radius_x = radius_x.abs();
        let mut radius_y = radius_y.abs();
        let p = (start - end).rotate(-rotation) / 2.0;
        {
            let s = (p.0 / radius_x).powi(2) + (p.1 / radius_y).powi(2);
            if 1.0 < s {
                radius_x *= s.sqrt();
                radius_y *= s.sqrt();
            }
        }
        let (rx2, ry2) = (radius_x.powi(2), radius_y.powi(2));
        let mut a = ((rx2 * ry2 - rx2 * p.1.powi(2) - ry2 * p.0.powi(2)) / (rx2 * p.1.powi(2) + ry2 * p.0.powi(2))).sqrt();
        if large == clockwise {
            a = -a;
        }
        let q = Point(radius_x * p.1 / radius_y, -radius_y * p.0 / radius_x) * a;
        let center = q.rotate(rotation) + (start + end) / 2.0;
        let a1 = Point((p.0 - q.0) / radius_x, (p.1 - q.1) / radius_y);
        let mut angle1 = (a1.0 / a1.norm()).acos().copysign(a1.1);
        let a2 = Point(-(p.0 + q.0) / radius_x, -(p.1 + q.1) / radius_y);
        let mut angle2 = (a2.0 / a2.norm()).acos().copysign(a2.1);
        if clockwise && angle2 < angle1 {
            angle2 += PI * 2.0;
        }
        if !clockwise && angle1 < angle2 {
            angle1 += PI * 2.0;
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

    pub fn quad(&mut self, control_x: f64, control_y: f64, x: f64, y: f64) {
        if let Some(last_pos) = self.last_pos {
            let quad = PathItem::Quad(Quad {
                start: last_pos,
                end: Point(x, y),
                control1: Point(control_x, control_y),
            });
            let left_point = quad.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
            self.set_pos(quad.right_point());
            self.push(quad);
        } else {
            panic!("PathBuilder::start() is required");
        }
    }

    pub fn cubic(
        &mut self,
        control_x1: f64,
        control_y1: f64,
        control_x2: f64,
        control_y2: f64,
        x: f64,
        y: f64,
    ) {
        if let Some(last_pos) = self.last_pos {
            let cubic = PathItem::Cubic(Cubic {
                start: last_pos,
                end: Point(x, y),
                control1: Point(control_x1, control_y1),
                control2: Point(control_x2, control_y2),
            });
            let left_point = cubic.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
            self.set_pos(cubic.right_point());
            self.push(cubic);
        } else {
            panic!("PathBuilder::start() is required");
        }
    }

    pub fn close(&mut self) {
        if let Some(p) = self.path_start {
            self.line_to(p.0, p.1);
            self.push(PathItem::CloseAndJump);
            self.path_start = None;
            self.last_pos = None;
        }
    }

    pub fn end(&mut self) -> Path {
        let mut items = Vec::new();
        std::mem::swap(&mut items, &mut self.items);
        Path(items)
    }

    pub fn current_pos(&self) -> Option<(f64, f64)> {
        self.last_pos.map(|p| p.into())
    }
}
