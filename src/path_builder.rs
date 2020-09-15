use crate::{
    models::{Line, Arc, Ellipse, Quad, Cubic},
    path3::{Path, PathItem},
    point::Point
};

#[derive(Debug, Clone)]
pub struct PathBuilder {
    items: Vec<PathItem>,
    last_pos: Option<Point>,
}

impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder {
            items: vec![],
            last_pos: None,
        }
    }

    fn push(&mut self, pi: PathItem) {
        if !pi.is_zero() {
            self.items.push(pi);
        }
    }

    pub fn start(&mut self, x: f64, y: f64) {
        assert!(self.items.is_empty());
        self.last_pos = Some(Point(x, y));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        let p = Point(x, y);
        if let Some(last_pos) = self.last_pos {
            self.push(PathItem::Line(Line(last_pos, p)));
        }
        self.last_pos = Some(p);
    }

    pub fn arc(&mut self, x: f64, y: f64, radius: f64, angle1: f64, angle2: f64) {
        let center = Point(x, y);
        let arc = PathItem::Arc(Arc{
            center, radius, angle1, angle2
        });
        if let Some(last_pos) = self.last_pos {
            let left_point = arc.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
        }
        self.last_pos = Some(arc.right_point());
        self.push(arc);
    }

    pub fn ellipse(&mut self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, angle1: f64, angle2: f64) {
        let center = Point(x, y);
        let ellipse = PathItem::Ellipse(Ellipse{
            center, radius_x, radius_y, rotation, angle1, angle2
        });
        if let Some(last_pos) = self.last_pos {
            let left_point = ellipse.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
        }
        self.last_pos = Some(ellipse.right_point());
        self.push(ellipse);
    }

    pub fn quad(&mut self, control_x: f64, control_y: f64, x: f64, y: f64) {
        if let Some(last_pos) = self.last_pos {
            let quad = PathItem::Quad(Quad{
                start: last_pos,
                end: Point(x, y),
                control1: Point(control_x, control_y),
            });
            let left_point = quad.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
            self.last_pos = Some(quad.right_point());
            self.push(quad);
        } else {
            panic!("PathBuilder::start() is required");
        }
    }

    pub fn cubic(&mut self, control_x1: f64, control_y1: f64, control_x2: f64, control_y2: f64, x: f64, y: f64) {
        if let Some(last_pos) = self.last_pos {
            let cubic = PathItem::Cubic(Cubic{
                start: last_pos,
                end: Point(x, y),
                control1: Point(control_x1, control_y1),
                control2: Point(control_x2, control_y2),
            });
            let left_point = cubic.left_point();
            if last_pos != left_point {
                self.push(PathItem::Line(Line(last_pos, left_point)));
            }
            self.last_pos = Some(cubic.right_point());
            self.push(cubic);
        } else {
            panic!("PathBuilder::start() is required");
        }
    }

    pub fn close(&mut self) {
        let p = self.items[0].left_point();
        self.line_to(p.0, p.1);
    }

    pub fn end(&self) -> Path {
        Path(self.items.clone())
    }
}
