use crate::point::Point;
use crate::path3::{Path, PathItem};
use crate::models::{Line, Arc};
use crate::geometry;
use crate::bold::path_item_bold;

#[derive(Debug, Clone)]
pub enum Join {
    Round,
    Bevel,
    Miter(f32),
    NoJoin,
}

#[derive(Debug, Clone)]
pub enum Cap {
    Round,
    Butt,
    Square,
}

pub fn path_outline(path: &Path, width: f64, join: &Join, cap: &Cap) -> Vec<PathItem> {
    let mut pis = Vec::with_capacity(path.0.len() * 4);
    path_item_bold(&mut pis, &path.0[0], width);
    let mut m = pis.len();
    for pi in path.0.iter().skip(1) {
        let l = pis.len();
        path_item_bold(&mut pis, pi, width);
        let p11 = pis[m - 2].right_point();
        let p12 = pis[l].left_point();
        let p21 = pis[l + 1].right_point();
        let p22 = pis[m - 1].left_point();
        let p0 = pi.left_point();
        add_join(&mut pis, join, p0, p11, p12, p21, p22);
        m = pis.len() - 2;
    }
    if path.is_closed() {
        let p11 = pis[m - 2].right_point();
        let p12 = pis[0].left_point();
        let p21 = pis[1].right_point();
        let p22 = pis[m - 1].left_point();
        let p0 = path.0[0].left_point();
        add_join(&mut pis, join, p0, p11, p12, p21, p22);
    } else {
        let p1 = pis[m - 2].right_point();
        let p2 = pis[m - 1].left_point();
        add_cap(&mut pis, cap, p1, p2);
        let p1 = pis[1].right_point();
        let p2 = pis[0].left_point();
        add_cap(&mut pis, cap, p1, p2);
    }
    pis
}

fn add_join(pis: &mut Vec<PathItem>, join: &Join, center: Point, start1: Point, end1: Point, start2: Point, end2: Point) {
    match join {
        Join::Round => {
            if geometry::point_is_right_side_of_line(start1 - center, end1 - center) {
                pis.push(PathItem::Arc(Arc::from_points(center, start1, end1)));
                pis.push(PathItem::Line(Line(start2, end2)));
            } else {
                pis.push(PathItem::Line(Line(start1, end1)));
                pis.push(PathItem::Arc(Arc::from_points(center, start2, end2)));
            }
        }
        Join::Bevel => {
            pis.push(PathItem::Line(Line(start1, end1)));
            pis.push(PathItem::Line(Line(start2, end2)));
        }
        Join::Miter(_) => {
            todo!()
        }
        Join::NoJoin => {
            pis.push(PathItem::Line(Line(start1, end2)));
            pis.push(PathItem::Line(Line(start2, end1)));
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
            let v = end - start;
            let v = Point(v.1, -v.0) * 0.5;
            pis.push(PathItem::Line(Line(start, start + v)));
            pis.push(PathItem::Line(Line(start + v, end + v)));
            pis.push(PathItem::Line(Line(end + v, end)));
        }
    }
}
