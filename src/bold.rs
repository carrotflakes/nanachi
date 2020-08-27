use crate::point::Point;
use crate::path3::{Path, PathItem};
use crate::models::{Line, Arc, Ellipse, Quad};

pub fn path_bold1(path: &Path, width: f64) -> Vec<PathItem> {
    let mut pis = Vec::with_capacity(path.0.len() * 4);
    let (pi1, pi2) = path_item_bold(&path.0[0], width);
    pis.push(pi1);
    pis.push(pi2);
    for pi in path.0.iter().skip(1) {
        let (pi1, pi2) = path_item_bold(pi, width);
        let p1 = pis[pis.len() - 2].right_point();
        let p2 = pis[pis.len() - 1].left_point();
        pis.push(PathItem::Line(Line(p1, pi1.left_point())));
        pis.push(PathItem::Line(Line(pi2.right_point(), p2)));
        pis.push(pi1);
        pis.push(pi2);
    }
    if path.is_closed() {
        let p1 = pis[pis.len() - 2].right_point();
        let p2 = pis[pis.len() - 1].left_point();
        pis.push(PathItem::Line(Line(p1, pis[0].left_point())));
        pis.push(PathItem::Line(Line(pis[1].right_point(), p2)));
    } else {
        let p1 = pis[pis.len() - 2].right_point();
        let p2 = pis[pis.len() - 1].left_point();
        pis.push(PathItem::Line(Line(p1, p2)));
        pis.push(PathItem::Line(Line(pis[1].right_point(), pis[0].left_point())));
    }
    pis
}

pub fn path_item_bold(path_item: &PathItem, width: f64) -> (PathItem, PathItem) {
    match path_item {
        PathItem::Line(Line(p1, p2)) => {
            let n = (*p2 - *p1).unit();
            let d = Point(n.1, -n.0) * width;
            (
                PathItem::Line(Line(*p1 + d, *p2 + d)),
                PathItem::Line(Line(*p2 - d, *p1 - d)),
            )
        }
        PathItem::Arc(arc) => {
            let signum = (arc.angle2 - arc.angle1).signum();
            (
                PathItem::Arc(Arc{
                    radius: (arc.radius + width * signum).max(0.0),
                    angle1: arc.angle1,
                    angle2: arc.angle2,
                    ..arc.clone()
                }),
                PathItem::Arc(Arc{
                    radius: (arc.radius - width * signum).max(0.0),
                    angle1: arc.angle2,
                    angle2: arc.angle1,
                    ..arc.clone()
                }),
            )
        }
        PathItem::Ellipse(ellipse) => {
            let signum = (ellipse.angle2 - ellipse.angle1).signum();
            (
                PathItem::Ellipse(Ellipse{
                    radius_x: (ellipse.radius_x + width * signum).max(0.0),
                    radius_y: (ellipse.radius_y + width * signum).max(0.0),
                    angle1: ellipse.angle1,
                    angle2: ellipse.angle2,
                    ..ellipse.clone()
                }),
                PathItem::Ellipse(Ellipse{
                    radius_x: (ellipse.radius_x - width * signum).max(0.0),
                    radius_y: (ellipse.radius_y - width * signum).max(0.0),
                    angle1: ellipse.angle2,
                    angle2: ellipse.angle1,
                    ..ellipse.clone()
                }),
            )
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
            (
                PathItem::Quad(Quad {
                    start: quad.start + start_d,
                    end: quad.end + end_d,
                    control1: quad.control1 + control1_d,
                }),
                PathItem::Quad(Quad {
                    start: quad.end - end_d,
                    end: quad.start - start_d,
                    control1: quad.control1 - control1_d,
                }),
            )
        }
    }
}
