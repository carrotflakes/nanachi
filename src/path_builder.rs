use crate::{
    models::{Line, Arc, Ellipse, Quad},
    path3::{Path, PathItem},
    point::Point
};

#[derive(Debug, Clone)]
pub struct PathBuilder {
    items: Vec<PathItem>,
    current_pos: Point,
}

impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder {
            items: vec![],
            current_pos: Point(0.0, 0.0),
        }
    }

    pub fn move_to(mut self, p: Point) -> PathBuilder {
        if !self.items.is_empty() {
            self.items.push(PathItem::Line(Line(self.current_pos, p)));
        }
        self.current_pos = p;
        self
    }

    pub fn line_to(mut self, p: Point) -> PathBuilder {
        if self.current_pos != p {
            self.items.push(PathItem::Line(Line(self.current_pos, p)));
            self.current_pos = p;
        }
        self
    }

    pub fn arc(mut self, center: Point, radius: f64, angle1: f64, angle2: f64) -> PathBuilder {
        let arc = PathItem::Arc(Arc{
            center, radius, angle1, angle2
        });
        if self.current_pos != arc.left_point() {
            self.items.push(PathItem::Line(Line(self.current_pos, arc.left_point())));
        }
        self.current_pos = arc.right_point();
        self.items.push(arc);
        self
    }

    pub fn ellipse(mut self, center: Point, radius_x: f64, radius_y: f64, rotation: f64, angle1: f64, angle2: f64) -> PathBuilder {
        let ellipse = PathItem::Ellipse(Ellipse{
            center, radius_x, radius_y, rotation, angle1, angle2
        });
        if self.current_pos != ellipse.left_point() {
            self.items.push(PathItem::Line(Line(self.current_pos, ellipse.left_point())));
        }
        self.current_pos = ellipse.right_point();
        self.items.push(ellipse);
        self
    }

    pub fn quad(mut self, control: Point, point: Point) -> PathBuilder {
        let quad = PathItem::Quad(Quad{
            start: self.current_pos,
            end: point,
            control1: control,
        });
        if self.current_pos != quad.left_point() {
            self.items.push(PathItem::Line(Line(self.current_pos, quad.left_point())));
        }
        self.current_pos = quad.right_point();
        self.items.push(quad);
        self
    }

    pub fn close(self) -> PathBuilder {
        let p = self.items[0].left_point();
        self.line_to(p)
    }

    pub fn end(self) -> Path {
        Path(self.items)
    }
}
