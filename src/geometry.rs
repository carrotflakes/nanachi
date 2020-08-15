use crate::point::Point;

pub fn intersect_line_and_horizon(a: Point, b: Point, hy: f64) -> f64 {
    assert!(a.1 != b.1);
    let r = (hy - a.1) / (b.1 - a.1);
    a.0 * (1.0 - r) + b.0 * r
}

pub fn intersect_line_and_vertical(a: Point, b: Point, vx: f64) -> f64 {
    assert!(a.0 != b.0);
    let r = (vx - a.0) / (b.0 - a.0);
    a.1 * (1.0 - r) + b.1 * r
}

pub fn intersect_segment_and_horizon(ax: f64, ay: f64, bx: f64, by: f64, hy: f64) -> Option<f64> {
    if ay != by && ((hy < ay) ^ (hy < by)) {
        let r = (hy - ay) / (by - ay);
        Some(ax * (1.0 - r) + bx * r)
    } else {
        None
    }
}

pub fn intersect_segment_and_vertical(ax: f64, ay: f64, bx: f64, by: f64, vx: f64) -> Option<f64> {
    if ax != bx && ((vx < ax) ^ (vx < bx)) {
        let r = (vx - ax) / (bx - ax);
        Some(ay * (1.0 - r) + by * r)
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

// pub fn intersect_circle_and_segment(p1: Point, p2: Point) -> Point {
//     let a = (p2.1 - p1.1) / (p2.0 - p1.0);
//     let b = p2.1 - p2.0 * a;
//     let c = (a * b) / (1.0 + a.powi(2)).sqrt();
//     let x = (1.0 - b.powi(2) + c.powi(2) - c) / (1.0 + a.powi(2)).sqrt();
//     let y = (1.0 - x.powi(2)).sqrt();
//     Point(x, y)
// }

pub fn intersect_circle_and_segment(p1: Point, p2: Point) -> Point {
    let det = p1.0 * p2.1 - p2.0 * p1.1;
    if det == 0.0 {
        return p2 / (p2.0 - p1.0).hypot(p2.1 - p1.1);
    }
    let a = (p2.1 - p1.1) / det;
    let b = (p1.0 - p2.0) / det;
    let c = a.powi(2) + b.powi(2);
    if 0.0 < det {
        let x = (a - b * (c - 1.0).sqrt()) / c;
        let y = (b + a * (c - 1.0).sqrt()) / c;
        Point(x, y)
    } else {
        let x = (a + b * (c - 1.0).sqrt()) / c;
        let y = (b - a * (c - 1.0).sqrt()) / c;
        Point(x, y)
    }
}

pub fn circle_2segment_area(p: Point, p1: Point, p2: Point) -> f64 { // p1 < p2
    let p1 = intersect_circle_and_segment(p, p1);
    let p2 = intersect_circle_and_segment(p, p2);
    let a1 = p1.1.atan2(p1.0);
    let a2 = p2.1.atan2(p2.0);
    let a3 = (a2 - a1).rem_euclid(std::f64::consts::PI * 2.0) / 2.0;
    match (point_is_right_side_of_line(p1, p), point_is_right_side_of_line(p2, p)) {
        (false, true) => a3 - tri_area(p, p2) - tri_area(p1, p), // inner
        (true, false) => a3 + tri_area(p2, p) + tri_area(p, p1), // outer
        (true, true) => { // right
            let p3 = intersect_line_and_line(Point(0.0, 0.0), p1, p, p2);
            a3 - tri_area(p3, p2) + tri_area2(p, p1, p3)
        },
        (false, false) => { // left
            let p3 = intersect_line_and_line(Point(0.0, 0.0), p2, p, p1);
            a3 - tri_area(p1, p3) + tri_area2(p, p3, p2)
        },
    }
}

pub fn circle_2segment_area_(p: Point, p1: Point, p2: Point) -> f64 {
    if p.norm() < 1.0 {
        circle_2segment_area(p, p1, p2)
    } else {
        fn f(d: f64) -> f64 {
            if d <= -1.0 {
                std::f64::consts::PI
            } else if 1.0 <= d {
                0.0
            } else {
                d.acos() - (1.0 - d * d).sqrt() * d
            }
        }
        let d1 = (p.0 * p1.1 - p.1 * p1.0) / (p.1 - p1.1).hypot(p.0 - p1.0);
        let d2 = (p.0 * p2.1 - p.1 * p2.0) / (p.1 - p2.1).hypot(p.0 - p2.0);
        f(d1) - f(d2)
    }
}

pub fn tri_area2(p: Point, p1: Point, p2: Point) -> f64 {
    tri_area(p1 - p, p2 - p)
}

pub fn tri_area(p1: Point, p2: Point) -> f64 {
    (p1.0 * p2.1 - p1.1 * p2.0) / 2.0
}

#[test]
fn test() {
    //assert_eq!(intersect_circle_and_segment(Point(0.1, 0.1), Point(4.0, 3.0)), Point(0.0, 0.0));
    //assert!((0.0..3.14 / 4.0).contains(&circle_2segment_area(Point(0.1, 0.1), Point(4.0, 3.0), Point(-3.0, 4.0))));
    //assert_eq!(circle_2segment_area(Point(-0.1, 0.1), Point(5.0, 3.0), Point(3.0, 5.0)), 0.0);
    //assert_eq!(circle_2segment_area(Point(-0.1, -0.1), Point(-3.0, -5.0), Point(-5.0, -3.0)), 0.0);
    // assert_eq!([circle_2segment_area(Point(0.1, 0.1), Point(4.0, 3.0), Point(-3.0, 4.0)),
    // circle_2segment_area(Point(0.1, 0.1), Point(-3.0, 4.0), Point(-4.0, -3.0)),
    // circle_2segment_area(Point(0.1, 0.1), Point(-4.0, -3.0), Point(3.0, -4.0)),
    // circle_2segment_area(Point(0.1, 0.1), Point(3.0, -4.0), Point(4.0, 3.0))], [0.0f64; 4]);
    assert_eq!((circle_2segment_area(Point(0.1, 0.1), Point(4.0, 3.0), Point(-3.0, 4.0)) +
    circle_2segment_area(Point(0.1, 0.1), Point(-3.0, 4.0), Point(-4.0, -3.0)) +
    circle_2segment_area(Point(0.1, 0.1), Point(-4.0, -3.0), Point(3.0, -4.0)) +
    circle_2segment_area(Point(0.1, 0.1), Point(3.0, -4.0), Point(4.0, 3.0)) - std::f64::consts::PI).abs(), 0.00001);
}
