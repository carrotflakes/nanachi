//! Primitive shape generators.

use crate::{
    models::Arc,
    path::{Path, PathItem},
    point::Point,
};
use std::f64::consts::PI;

/// Create a circle.
pub fn circle(x: f64, y: f64, radius: f64) -> Path {
    Path::new(vec![
        PathItem::Arc(Arc {
            center: Point(x, y),
            radius,
            angle1: 0.0,
            angle2: PI * 2.0,
        }),
        PathItem::CloseAndJump,
    ])
}

/// Create a rectangle.
pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Path {
    Path::from_points(
        &vec![
            Point(x, y),
            Point(x, y + height),
            Point(x + width, y + height),
            Point(x + width, y),
            Point(x, y),
        ],
        true,
    )
}

/// Create a triangle.
pub fn triangle(size: f64) -> Path {
    ngon(3, size)
}

/// Create a regular n-gon.
pub fn ngon(n: usize, size: f64) -> Path {
    assert!(3 <= n);
    let p = Point(0.0, -size);
    Path::from_points(
        &(0..=n)
            .map(|i| p.rotate(i as f64 / n as f64 * std::f64::consts::PI * 2.0))
            .collect::<Vec<_>>(),
        true,
    )
}
