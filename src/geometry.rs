use crate::point::Point;

pub fn intersect_segment_and_horizon(ax: f64, ay: f64, bx: f64, by: f64, hy: f64) -> Option<f64> {
    if ay != by && ((hy < ay) ^ (hy < by)) {
        let r = (hy - ay) / (by - ay);
        Some(ax * (1.0 - r) + bx * r)
    } else {
        None
    }
}

pub fn squared_distance_between_line_segment_and_point<P1: Into<Point>, P2: Into<Point>>(
    p1: P1,
    p2: P1,
    p0: P2,
) -> f64 {
    let p1: Point = p1.into();
    let p2: Point = p2.into();
    let p0: Point = p0.into();
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let a2 = a.powi(2);
    let b2 = b.powi(2);
    let r2 = a2 + b2;
    let tt = -(a * (p1.0 - p0.0) + b * (p1.1 - p0.1));
    if tt < 0.0 {
        (p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)
    } else if tt > r2 {
        (p2.0 - p0.0).powi(2) + (p2.1 - p0.1).powi(2)
    } else {
        (a * (p1.1 - p0.1) - b * (p1.0 - p0.0)).powi(2) / r2
    }
}

pub fn intersect_line_and_line(p1: Point, p2: Point, p3: Point, p4: Point) -> Point {
  let det = (p1.0 - p2.0) * (p4.1 - p3.1) - (p4.0 - p3.0) * (p1.1 - p2.1);
  let t = ((p4.1 - p3.1) * (p4.0 - p2.0) + (p3.0 - p4.0) * (p4.1 - p2.1)) / det;
  let x = t * p1.0 + (1.0 - t) * p2.0;
  let y = t * p1.1 + (1.0 - t) * p2.1;
  Point(x, y)
}

pub fn point_is_right_side_of_line(p1: Point, p2: Point) -> bool {
    p1.0 * p2.1 < p1.1 * p2.0
}
