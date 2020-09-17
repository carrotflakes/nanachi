use crate::{legacy_path::Path, point::Point};

pub fn square(size: f64) -> Path {
    vec![
        Point(-size, -size),
        Point(-size, size),
        Point(size, size),
        Point(size, -size),
        Point(-size, -size),
    ]
    .into()
}

pub fn rect(width: f64, height: f64) -> Path {
    vec![
        Point(-width, -height),
        Point(-width, height),
        Point(width, height),
        Point(width, -height),
        Point(-width, -height),
    ]
    .into()
}

pub fn triangle(size: f64) -> Path {
    ngon(3, size)
}

pub fn ngon(n: usize, size: f64) -> Path {
    assert!(3 <= n);
    let p = Point(0.0, -size);
    let mut vec = Vec::with_capacity(n);
    vec.push(p);
    for i in 1..n {
        vec.push(rotate_point(
            p,
            i as f64 / n as f64 * std::f64::consts::PI * 2.0,
        ));
    }
    vec.push(p);
    vec.into()
}

fn rotate_point(p: Point, rad: f64) -> Point {
    let (sin, cos) = rad.sin_cos();
    Point(p.0 * cos - p.1 * sin, p.0 * sin + p.1 * cos)
}
