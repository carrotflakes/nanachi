use crate::point::Point;
use crate::models::{Line, Arc, Ellipse, Quad};

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
                        -arc.angle2.sin() * arc.radius,
                    )
            }
            PathItem::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle2.cos() * ellipse.radius_x;
                let y = -ellipse.angle2.sin() * ellipse.radius_y;
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
                        -arc.angle1.sin() * arc.radius,
                    )
            }
            PathItem::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle1.cos() * ellipse.radius_x;
                let y = -ellipse.angle1.sin() * ellipse.radius_y;
                ellipse.center + Point(x * cos - y * sin, x * sin + y * cos)
            }
            PathItem::Quad(quad) => {
                quad.start
            }
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
}
