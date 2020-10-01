use crate::{path::Path, point::Point};

pub fn square(x: f64, y: f64, size: f64) -> Path {
    rect(x, y, size, size)
}

pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Path {
    Path::from_points(&vec![
        Point(x - width, y - height),
        Point(x - width, y + height),
        Point(x + width, y + height),
        Point(x + width, y - height),
        Point(x - width, y - height),
    ], true)
}

pub fn triangle(size: f64) -> Path {
    ngon(3, size)
}

pub fn ngon(n: usize, size: f64) -> Path {
    assert!(3 <= n);
    let p = Point(0.0, -size);
    Path::from_points(&(0..=n).map(|i| p.rotate(i as f64 / n as f64 * std::f64::consts::PI * 2.0)).collect::<Vec<_>>(), true)
}
