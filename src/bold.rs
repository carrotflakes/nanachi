use crate::point::Point;
use crate::path3::{Path, PathItem};
use crate::models::{Line, Arc, Ellipse, Quad};

pub fn path_bold(path: &Path, width: f64) -> Path {
    Path(path.0.iter().flat_map(|pi| path_item_bold(pi, width)).collect())
}

pub fn path_item_bold(path_item: &PathItem, width: f64) -> Vec<PathItem> {
    match path_item {
        PathItem::Line(Line(p1, p2)) => {
            let n = (*p2 - *p1).unit();
            let d = Point(n.1, -n.0) * width;
            vec![
                PathItem::Line(Line(*p1 + d, *p2 + d)),
                PathItem::Line(Line(*p2 + d, *p2 - d)),
                PathItem::Line(Line(*p2 - d, *p1 - d)),
                PathItem::Line(Line(*p1 - d, *p1 + d)),
            ]
        }
        PathItem::Arc(arc) => {
            let (a1, a2) = arc.angle_norm();
            let outer_radius = arc.radius + width;
            let inner_radius = (arc.radius - width).max(0.0);
            vec![
                PathItem::Arc(Arc{
                    radius: outer_radius,
                    angle1: a1,
                    angle2: a2,
                    ..arc.clone()
                }),
                PathItem::Line(Line(
                    arc.center + Point(a2.cos(), a2.sin()) * outer_radius,
                    arc.center + Point(a2.cos(), a2.sin()) * inner_radius,
                )),
                PathItem::Arc(Arc{
                    radius: inner_radius,
                    angle1: a2,
                    angle2: a1,
                    ..arc.clone()
                }),
                PathItem::Line(Line(
                    arc.center + Point(a1.cos(), a1.sin()) * inner_radius,
                    arc.center + Point(a1.cos(), a1.sin()) * outer_radius,
                )),
            ]
        }
        PathItem::Ellipse(ellipse) => {
            let (a1, a2) = ellipse.angle_norm();
            let outer_ellipse = PathItem::Ellipse(Ellipse{
                radius_x: ellipse.radius_x + width,
                radius_y: ellipse.radius_y + width,
                angle1: a1,
                angle2: a2,
                ..ellipse.clone()
            });
            let inner_ellipse = PathItem::Ellipse(Ellipse{
                radius_x: (ellipse.radius_x - width).max(0.0),
                radius_y: (ellipse.radius_y - width).max(0.0),
                angle1: a2,
                angle2: a1,
                ..ellipse.clone()
            });
            let line1 = PathItem::Line(Line(
                outer_ellipse.right_point(),
                inner_ellipse.left_point(),
            ));
            let line2 = PathItem::Line(Line(
                inner_ellipse.right_point(),
                outer_ellipse.left_point(),
            ));
            vec![outer_ellipse, line1,inner_ellipse, line2]
        }
        PathItem::Quad(quad) => {
            let start_d = {
                let n = (quad.control1 - quad.start).unit();
                Point(n.1, -n.0) * width
            };
            let end_d = {
                let n = (quad.end - quad.control1).unit();
                Point(n.1, -n.0) * width
            };
            let control1_d = {
                let c = (quad.end + quad.start) / 2.0;
                let n = (quad.control1 - c).unit();
                let m = quad.end - quad.start;
                let m = (quad.control1 + Point(m.1, -m.0)).unit();
                (n + m) * width
            };
            let outer_quad = PathItem::Quad(Quad {
                start: quad.start + start_d,
                end: quad.end + end_d,
                control1: quad.control1 + control1_d,
            });
            let inner_quad = PathItem::Quad(Quad {
                start: quad.end - end_d,
                end: quad.start - start_d,
                control1: quad.control1 - control1_d,
            });
            vec![
                outer_quad,
                PathItem::Line(Line(quad.end + end_d, quad.end - end_d)),
                inner_quad,
                PathItem::Line(Line(quad.start - start_d, quad.start + start_d)),
            ]
        }
    }
}
