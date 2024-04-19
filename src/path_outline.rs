//! Generate path outline.

use crate::models::{Arc, Ellipse, Line, Quad};
use crate::path::{Path, PathItem};
use crate::point::Point;

/// Join types for [`path_outline`]
#[derive(Debug, Clone)]
pub enum Join {
    Round,
    Bevel,
    Miter(f32),
}

/// Cap types for [`path_outline`]
#[derive(Debug, Clone)]
pub enum Cap {
    Round,
    Butt,
    Square,
}

/// Create a path that is outline of given path.
pub fn path_outline(path: &Path, width: f64, join: &Join, cap: &Cap) -> Path {
    assert_ne!(width, 0.0);
    let mut res = Vec::with_capacity(path.0.len() * 4);
    let mut tmp = Vec::with_capacity(4);
    for (pis, closed) in path.continuations() {
        if closed {
            // outer
            let mut it = pis.iter().filter(|pi| !pi.is_zero());
            let m = res.len();
            if let Some(pi) = it.next() {
                path_item_offset(&mut res, pi, width);
            } else {
                continue;
            }
            let first = res[m].left_point();
            for pi in it {
                path_item_offset(&mut tmp, pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let s = res.last().unwrap().right_point();
            add_join(&mut res, join, pis[0].left_point(), s, first);
            res.push(PathItem::CloseAndJump);

            // inner
            let mut it = pis
                .iter()
                .rev()
                .filter(|pi| !pi.is_zero())
                .map(|pi| pi.flip());
            let m = res.len();
            path_item_offset(&mut res, &it.next().unwrap(), width);
            let first = res[m].left_point();
            for pi in it {
                path_item_offset(&mut tmp, &pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let s = res.last().unwrap().right_point();
            add_join(&mut res, join, pis[0].left_point(), s, first);
            res.push(PathItem::CloseAndJump);
        } else {
            let mut it = pis.iter().filter(|pi| !pi.is_zero());
            let m = res.len();
            if let Some(pi) = it.next() {
                path_item_offset(&mut res, pi, width);
            } else {
                continue;
            }
            let first = res[m].left_point();
            for pi in it {
                path_item_offset(&mut tmp, pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let mut it = pis
                .iter()
                .rev()
                .filter(|pi| !pi.is_zero())
                .map(|pi| pi.flip());
            path_item_offset(&mut tmp, &it.next().unwrap(), width);
            let s = res.last().unwrap().right_point();
            add_cap(&mut res, cap, s, tmp[0].left_point());
            res.extend(tmp.drain(..));
            for pi in it {
                path_item_offset(&mut tmp, &pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let s = res.last().unwrap().right_point();
            add_cap(&mut res, cap, s, first);
            res.push(PathItem::CloseAndJump);
        }
    }
    Path(res)
}

pub fn path_offset(path: &Path, width: f64, join: &Join) -> Path {
    let mut res = Vec::with_capacity(path.0.len() * 2);
    let mut tmp = Vec::with_capacity(4);
    for (pis, closed) in path.continuations() {
        if !closed {
            todo!("path_offset not supports unclosed path");
        }
        let mut it = pis.iter().filter(|pi| !pi.is_zero());
        let m = res.len();
        if let Some(pi) = it.next() {
            path_item_offset(&mut res, pi, width);
        } else {
            continue;
        }
        let first = res[m].left_point();
        for pi in it {
            path_item_offset(&mut tmp, pi, width);
            let s = res.last().unwrap().right_point();
            add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
            res.extend(tmp.drain(..));
        }
        let s = res.last().unwrap().right_point();
        add_join(&mut res, join, pis[0].left_point(), s, first);
        res.push(PathItem::CloseAndJump);
    }
    Path(res)
}

fn add_join(pis: &mut Vec<PathItem>, join: &Join, center: Point, start: Point, end: Point) {
    if start == end {
        return;
    }
    let mut bevel = || {
        pis.push(PathItem::Line(Line(start, end)));
    };
    match join {
        Join::Round => {
            if point_is_right_side_of_line(start - center, end - center) {
                bevel();
            } else {
                pis.push(PathItem::Arc(Arc::from_points(center, start, end)));
            }
        }
        Join::Bevel => {
            bevel();
        }
        Join::Miter(limit) => {
            if point_is_right_side_of_line(start - center, end - center) {
                bevel();
            } else {
                let p = intersect_line_and_line(
                    start,
                    start + Point::from((start.y() - center.y(), center.x() - start.x())),
                    end,
                    end + Point::from((center.y() - end.y(), end.x() - center.x())),
                );
                if ((p - center).norm() as f32) < *limit {
                    pis.push(PathItem::Line(Line(start, p)));
                    pis.push(PathItem::Line(Line(p, end)));
                } else {
                    bevel();
                }
            }
        }
    }
}

fn add_cap(pis: &mut Vec<PathItem>, cap: &Cap, start: Point, end: Point) {
    match cap {
        Cap::Round => {
            pis.push(PathItem::Arc(Arc::from_points(
                (start + end) / 2.0,
                start,
                end,
            )));
        }
        Cap::Butt => {
            pis.push(PathItem::Line(Line(start, end)));
        }
        Cap::Square => {
            let v = Point::from((end.y() - start.y(), start.x() - end.x())) * 0.5;
            pis.push(PathItem::Line(Line(start, start + v)));
            pis.push(PathItem::Line(Line(start + v, end + v)));
            pis.push(PathItem::Line(Line(end + v, end)));
        }
    }
}

fn path_item_offset(pis: &mut Vec<PathItem>, path_item: &PathItem, width: f64) {
    match path_item {
        PathItem::Line(Line(p1, p2)) => {
            let n = (*p2 - *p1).unit();
            let d = Point::from((n.y(), -n.x())) * width;
            pis.push(PathItem::Line(Line(*p1 + d, *p2 + d)));
        }
        PathItem::Arc(arc) => {
            let signum = (arc.angle2 - arc.angle1).signum();
            pis.push(PathItem::Arc(Arc {
                radius: (arc.radius + width * signum).max(0.0),
                angle1: arc.angle1,
                angle2: arc.angle2,
                ..arc.clone()
            }));
        }
        PathItem::Ellipse(ellipse) => {
            let signum = (ellipse.angle2 - ellipse.angle1).signum();
            pis.push(PathItem::Ellipse(Ellipse {
                radius_x: (ellipse.radius_x + width * signum).max(0.0),
                radius_y: (ellipse.radius_y + width * signum).max(0.0),
                angle1: ellipse.angle1,
                angle2: ellipse.angle2,
                ..ellipse.clone()
            }));
        }
        PathItem::Quad(quad) => {
            let start_d = {
                let n = (quad.control1 - quad.start).unit();
                Point::from((n.y(), -n.x())) * width
            };
            let end_d = {
                let n = (quad.end - quad.control1).unit();
                Point::from((n.y(), -n.x())) * width
            };
            if {
                let v0 = quad.start - quad.control1;
                let v1 = quad.end - quad.control1;
                0.0 <= v1.x() * v0.x() + v1.y() * v0.y() // whether acute angle
            } {
                let t = quad.closest_t_to_control();
                let (q1, q2) = quad.separate(t);
                let middle_d = {
                    let n = (q2.control1 - q2.start).unit();
                    Point::from((n.y(), -n.x())) * width
                };

                pis.push(PathItem::Quad(Quad {
                    start: q1.start + start_d,
                    end: q1.end + middle_d,
                    control1: intersect_line_and_line(
                        q1.start + start_d,
                        q1.control1 + start_d,
                        q1.end + middle_d,
                        q1.control1 + middle_d,
                    ),
                }));
                pis.push(PathItem::Quad(Quad {
                    start: q2.start + middle_d,
                    end: q2.end + end_d,
                    control1: intersect_line_and_line(
                        q2.start + middle_d,
                        q2.control1 + middle_d,
                        q2.end + end_d,
                        q2.control1 + end_d,
                    ),
                }));
            } else {
                pis.push(PathItem::Quad(Quad {
                    start: quad.start + start_d,
                    end: quad.end + end_d,
                    control1: intersect_line_and_line(
                        quad.start + start_d,
                        quad.control1 + start_d,
                        quad.end + end_d,
                        quad.control1 + end_d,
                    ),
                }));
            }
        }
        PathItem::Cubic(_) => panic!("path_outline not support cubic curve."),
        _ => unreachable!(),
    }
}

fn intersect_line_and_line(p1: Point, p2: Point, p3: Point, p4: Point) -> Point {
    let det = (p1.x() - p2.x()) * (p4.y() - p3.y()) - (p4.x() - p3.x()) * (p1.y() - p2.y());
    let t = ((p4.y() - p3.y()) * (p4.x() - p2.x()) + (p3.x() - p4.x()) * (p4.y() - p2.y())) / det;
    let x = t * p1.x() + (1.0 - t) * p2.x();
    let y = t * p1.y() + (1.0 - t) * p2.y();
    Point::from((x, y))
}

fn point_is_right_side_of_line(p1: Point, p2: Point) -> bool {
    p1.x() * p2.y() < p1.y() * p2.x()
}
