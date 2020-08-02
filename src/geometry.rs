use crate::point::Point;

pub fn intersect_segment_and_horizon(ax: f64, ay: f64, bx: f64, by: f64, hy: f64) -> Option<f64> {
    if ay != by && ((hy < ay) ^ (hy < by)) {
        let r = (hy - ay) / (by - ay);
        Some(ax * (1.0 - r) + bx * r)
    } else {
        None
    }
}

pub fn transform(
    p: &(f64, f64),
    translation: (f64, f64),
    rotation: f64,
    scale: (f64, f64),
) -> (f64, f64) {
    let (x, y) = p;
    let (sin, cos) = rotation.sin_cos();
    let (x, y) = (x * cos - y * sin, x * sin + y * cos);
    let (x, y) = (x * scale.0, y * scale.1);
    let (x, y) = (x + translation.0, y + translation.1);
    (x, y)
}

pub fn distance_between_line_and_point<P1: Into<Point>, P2: Into<Point>>(
    p1: P1,
    p2: P1,
    p0: P2,
) -> f64 {
    let p1: Point = p1.into();
    let p2: Point = p2.into();
    let p0: Point = p0.into();
    ((p2.1 - p1.1) * p0.0 - (p2.0 - p1.0) * p0.1 + p2.0 * p1.1 - p2.1 * p1.0).abs()
        / (p2.1 - p1.1).hypot(p2.0 - p1.0)
}

pub fn distance_between_line_segment_and_point<P1: Into<Point>, P2: Into<Point>>(
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
