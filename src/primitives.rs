//! Primitive shape generators.

use crate::{
    models::Arc,
    path::{Path, PathItem},
    point::Point,
};
use std::f32::consts::TAU;

/// Create a circle.
pub fn circle(x: f32, y: f32, radius: f32) -> Path {
    Path::new(vec![
        PathItem::Arc(Arc {
            center: Point([x, y]),
            radius,
            angle1: 0.0,
            angle2: TAU,
        }),
        PathItem::CloseAndJump,
    ])
}

/// Create a rectangle.
pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Path {
    Path::from_points(
        &vec![
            Point([x, y]),
            Point([x, y + height]),
            Point([x + width, y + height]),
            Point([x + width, y]),
            Point([x, y]),
        ],
        true,
    )
}

/// Create a triangle.
pub fn triangle(x: f32, y: f32, size: f32) -> Path {
    ngon(x, y, 3, size)
}

/// Create a regular n-gon.
pub fn ngon(x: f32, y: f32, n: usize, size: f32) -> Path {
    assert!(3 <= n);
    let center = Point([x, y]);
    let p = Point([0.0, -size]);
    Path::from_points(
        &(0..=n)
            .map(|i| center + p.rotate(i as f32 / n as f32 * TAU))
            .collect::<Vec<_>>(),
        true,
    )
}
