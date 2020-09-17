use crate::point::Point;
use crate::path3::{Path, PathItem};
use crate::models::{Line, Arc, Ellipse, Quad};
use crate::geometry;

#[derive(Debug, Clone)]
pub enum Join {
    Round,
    Bevel,
    Miter(f32),
}

#[derive(Debug, Clone)]
pub enum Cap {
    Round,
    Butt,
    Square,
}

pub fn path_outline(path: &Path, width: f64, join: &Join, cap: &Cap) -> Vec<PathItem> {
    assert_ne!(width, 0.0);
    let pis = &path.0;
    let mut i = 0;
    let mut res = Vec::with_capacity(pis.len() * 4);
    let mut tmp = Vec::with_capacity(4);
    while i < pis.len() {
        let mut closed = false;
        let mut j = pis.len();
        for (k, pi) in pis.iter().skip(i).enumerate() {
            match pi {
                PathItem::CloseAndJump => {
                    closed = true;
                    j = i + k;
                    break;
                }
                PathItem::Jump => {
                    j = i + k;
                    break;
                }
                _ => {}
            }
        }

        let pis = &pis[i..j];
        if closed {
            // outer
            let m = res.len();
            path_item_offset(&mut res, &pis[0], width);
            let first = res[m].left_point();
            for pi in pis.iter().skip(1) {
                path_item_offset(&mut tmp, pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let s = res.last().unwrap().right_point();
            add_join(&mut res, join, pis[0].left_point(), s, first);
            res.push(PathItem::CloseAndJump);

            // inner
            let m = res.len();
            path_item_offset(&mut res, &pis.last().unwrap().flip(), width);
            let first = res[m].left_point();
            for pi in pis.iter().rev().skip(1) {
                let pi = &pi.flip();
                path_item_offset(&mut tmp, pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let s = res.last().unwrap().right_point();
            add_join(&mut res, join, pis[0].left_point(), s, first);
            res.push(PathItem::CloseAndJump);
        } else {
            let m = res.len();
            path_item_offset(&mut res, &pis[0], width);
            let first = res[m].left_point();
            for pi in pis.iter().skip(1) {
                path_item_offset(&mut tmp, pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            path_item_offset(&mut tmp, &pis.last().unwrap().flip(), width);
            let s = res.last().unwrap().right_point();
            add_cap(&mut res, cap, s, tmp[0].left_point());
            res.extend(tmp.drain(..));
            for pi in pis.iter().rev().skip(1) {
                let pi = &pi.flip();
                path_item_offset(&mut tmp, pi, width);
                let s = res.last().unwrap().right_point();
                add_join(&mut res, join, pi.left_point(), s, tmp[0].left_point());
                res.extend(tmp.drain(..));
            }
            let s = res.last().unwrap().right_point();
            add_cap(&mut res, cap, s, first);
            res.push(PathItem::CloseAndJump);
        }
        i = j + 1;
    }
    res
}

fn add_join(pis: &mut Vec<PathItem>, join: &Join, center: Point, start1: Point, end1: Point) {
    let mut bevel = || {
        pis.push(PathItem::Line(Line(start1, end1)));
    };
    match join {
        Join::Round => {
            if geometry::point_is_right_side_of_line(start1 - center, end1 - center) {
                pis.push(PathItem::Line(Line(start1, end1)));
            } else {
                pis.push(PathItem::Arc(Arc::from_points(center, start1, end1)));
            }
        }
        Join::Bevel => {
            bevel();
        }
        Join::Miter(limit) => {
            if geometry::point_is_right_side_of_line(start1 - center, end1 - center) {
                bevel();
            } else {
                let p = geometry::intersect_line_and_line(
                    start1,
                    start1 + Point(start1.1 - center.1, center.0 - start1.0),
                    end1,
                    end1 + Point(center.1 - end1.1, end1.0 - center.0),
                );
                if ((p - center).norm() as f32) < *limit {
                    pis.push(PathItem::Line(Line(start1, p)));
                    pis.push(PathItem::Line(Line(p, end1)));
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
            pis.push(PathItem::Arc(Arc::from_points((start + end) / 2.0, start, end)));
        }
        Cap::Butt => {
            pis.push(PathItem::Line(Line(start, end)));
        }
        Cap::Square => {
            let v = Point(end.1 - start.1, start.0 - end.0) * 0.5;
            pis.push(PathItem::Line(Line(start, start + v)));
            pis.push(PathItem::Line(Line(start + v, end + v)));
            pis.push(PathItem::Line(Line(end + v, end)));
        }
    }
}

pub fn path_item_offset(pis: &mut Vec<PathItem>, path_item: &PathItem, width: f64) {
    match path_item {
        PathItem::Line(Line(p1, p2)) => {
            let n = (*p2 - *p1).unit();
            let d = Point(n.1, -n.0) * width;
            pis.push(PathItem::Line(Line(*p1 + d, *p2 + d)));
        }
        PathItem::Arc(arc) => {
            let signum = (arc.angle2 - arc.angle1).signum();
            pis.push(PathItem::Arc(Arc{
                radius: (arc.radius + width * signum).max(0.0),
                angle1: arc.angle1,
                angle2: arc.angle2,
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
            if {
                let v0 = quad.start - quad.control1;
                let v1 = quad.end - quad.control1;
                0.0 <= v1.0 * v0.0 + v1.1 * v0.1 // whether acute angle
             } {
                let t= quad.closest_t_to_control();
                let (q1, q2) = quad.separate(t);
                let middle_d = {
                    let n = (q2.control1 - q2.start).unit();
                    Point(n.1, -n.0) * width
                };

                pis.push(PathItem::Quad(Quad {
                    start: q1.start + start_d,
                    end: q1.end + middle_d,
                    control1: geometry::intersect_line_and_line(
                        q1.start + start_d, q1.control1 + start_d,
                        q1.end + middle_d, q1.control1 + middle_d),
                }));
                pis.push(PathItem::Quad(Quad {
                    start: q1.end - middle_d,
                    end: q1.start - start_d,
                    control1: geometry::intersect_line_and_line(
                        q1.start - start_d, q1.control1 - start_d,
                        q1.end - middle_d, q1.control1 - middle_d),
                }));
            } else {
                pis.push(PathItem::Quad(Quad {
                    start: quad.start + start_d,
                    end: quad.end + end_d,
                    control1: geometry::intersect_line_and_line(
                        quad.start + start_d, quad.control1 + start_d,
                        quad.end + end_d, quad.control1 + end_d),
                }));
            }
        }
        PathItem::Cubic(_) => {panic!("path_outline not support cubic curve.")}
        _ => {unreachable!()}
    }
}
