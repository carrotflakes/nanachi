use crate::point::Point;
use crate::path3::{Path, PathItem};
use crate::models::{Line, Arc, Ellipse, Quad};
use crate::geometry;

pub fn path_bold1(path: &Path, width: f64) -> Vec<PathItem> {
    let mut pis = Vec::with_capacity(path.0.len() * 4);
    path_item_bold(&mut pis, &path.0[0], width);
    let mut m = pis.len();
    for pi in path.0.iter().skip(1) {
        let l = pis.len();
        path_item_bold(&mut pis, pi, width);
        let p1 = pis[m - 2].right_point();
        let p2 = pis[m - 1].left_point();
        pis.push(PathItem::Line(Line(p1, pis[l].left_point())));
        pis.push(PathItem::Line(Line(pis[l + 1].right_point(), p2)));
        m = pis.len() - 2;
    }
    if path.is_closed() {
        let p1 = pis[m - 2].right_point();
        let p2 = pis[m - 1].left_point();
        pis.push(PathItem::Line(Line(p1, pis[0].left_point())));
        pis.push(PathItem::Line(Line(pis[1].right_point(), p2)));
    } else {
        let p1 = pis[m - 2].right_point();
        let p2 = pis[m - 1].left_point();
        pis.push(PathItem::Line(Line(p1, p2)));
        pis.push(PathItem::Line(Line(pis[1].right_point(), pis[0].left_point())));
    }
    pis
}

pub fn path_bold2(path: &Path, width: f64) -> Vec<PathItem> {
    let mut pis = Vec::with_capacity(path.0.len() * 4);
    for pi in path.0.iter() {
        let m = pis.len();
        path_item_bold(&mut pis, pi, width);
        let n = pis.len();
        pis.push(PathItem::Line(Line(pis[n - 2].right_point(), pis[n - 1].left_point())));
        pis.push(PathItem::Line(Line(pis[m + 1].right_point(), pis[m].left_point())));
    }
    pis
}

pub fn path_item_bold(pis: &mut Vec<PathItem>, path_item: &PathItem, width: f64) {
    match path_item {
        PathItem::Line(Line(p1, p2)) => {
            let n = (*p2 - *p1).unit();
            let d = Point(n.1, -n.0) * width;
            pis.push(PathItem::Line(Line(*p1 + d, *p2 + d)));
            pis.push(PathItem::Line(Line(*p2 - d, *p1 - d)));
        }
        PathItem::Arc(arc) => {
            let signum = (arc.angle2 - arc.angle1).signum();
            pis.push(PathItem::Arc(Arc{
                radius: (arc.radius + width * signum).max(0.0),
                angle1: arc.angle1,
                angle2: arc.angle2,
                ..arc.clone()
            }));
            pis.push(PathItem::Arc(Arc{
                radius: (arc.radius - width * signum).max(0.0),
                angle1: arc.angle2,
                angle2: arc.angle1,
                ..arc.clone()
            }));
        }
        PathItem::Ellipse(ellipse) => {
            let signum = (ellipse.angle2 - ellipse.angle1).signum();
            pis.push(PathItem::Ellipse(Ellipse{
                radius_x: (ellipse.radius_x + width * signum).max(0.0),
                radius_y: (ellipse.radius_y + width * signum).max(0.0),
                angle1: ellipse.angle1,
                angle2: ellipse.angle2,
                ..ellipse.clone()
            }));
            pis.push(PathItem::Ellipse(Ellipse{
                radius_x: (ellipse.radius_x - width * signum).max(0.0),
                radius_y: (ellipse.radius_y - width * signum).max(0.0),
                angle1: ellipse.angle2,
                angle2: ellipse.angle1,
                ..ellipse.clone()
            }));
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
            pis.push(PathItem::Quad(Quad {
                start: quad.start + start_d,
                end: quad.end + end_d,
                control1: geometry::intersect_line_and_line(
                    quad.start + start_d, quad.control1 + start_d,
                    quad.end + end_d, quad.control1 + end_d),
            }));
            pis.push(PathItem::Quad(Quad {
                start: quad.end - end_d,
                end: quad.start - start_d,
                control1: geometry::intersect_line_and_line(
                    quad.start - start_d, quad.control1 - start_d,
                    quad.end - end_d, quad.control1 - end_d),
            }));
        }
    }
}
