use crate::{
    models::{Line, Arc, Ellipse, Quad, Cubic},
    path3::{Path, PathItem},
    point::Point
};

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
            Some(PathItem::CloseAndJump) | Some(PathItem::Jump) | None => {},
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
        let arc = PathItem::Arc(Arc{
            center, radius, angle1, angle2
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
        self.set_pos(ellipse.right_point());
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
            self.set_pos(quad.right_point());
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
}
