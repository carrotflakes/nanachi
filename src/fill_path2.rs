use crate::geometry;
use crate::path2::PathEdge;
use crate::point::Point;
use crate::position_color::PositionColor;
use image::{ImageBuffer, Pixel};
use std::f64::{
    consts::{FRAC_PI_2, PI},
    NAN,
};

pub fn draw_fill<X, C: PositionColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    edges: &Vec<PathEdge>,
    position_color: &C,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    for y in 0..img.height() {
        for x in 0..img.width() {
            let r: f64 = edges
                .iter()
                .map(|e| match e {
                    PathEdge::Line(p1, p2) => {
                        area(*p1, *p2, y as f64, x as f64 + 1.0)
                            - area(*p1, *p2, y as f64, x as f64)
                    }
                    PathEdge::Arc {
                        center,
                        radius,
                        angle1,
                        angle2,
                    } => 0.0,
                })
                .sum();
            img_blend_pixel(img, position_color, x as i32, y as i32, r.min(1.0).max(0.0));
        }
    }
}

fn scan(edges: &Vec<PathEdge>, y: f64) -> Vec<f64> {
    let mut vec = Vec::new();
    for edge in edges {
        match edge {
            PathEdge::Line(p1, p2) => {
                vec.push(
                    geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y)
                        .unwrap_or(NAN),
                );
            }
            PathEdge::Arc {
                center,
                radius,
                angle1,
                angle2,
            } => {
                // ittann mushi
                vec.push(NAN);
            }
        }
    }
    vec
}

fn area(p1: Point, p2: Point, y: f64, x: f64) -> f64 {
    // // giji
    // if let Some(xx) = geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y) {
    //     xx.min(x).copysign(p1.1 - p2.1)
    // } else {
    //     0.0
    // }
    match (
        geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y),
        geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y + 1.0),
    ) {
        (Some(x1), Some(x2)) => match (x < x1, x < x2) {
            (true, true) => x,
            (false, true) => x - (x - x1).powi(2) / (x2 - x1) / 2.0,
            (true, false) => x - (x - x2).powi(2) / (x1 - x2) / 2.0,
            (false, false) => (x1 + x2) / 2.0,
        },
        (Some(x1), None) => {
            if y <= p1.1 {
                (p1.1 - y)
                    * if x < x1.min(p1.0) {
                        x
                    } else if x < x1.max(p1.0) {
                        x - (x - x1.min(p1.0)).powi(2) / (x1 - p1.0).abs() / 2.0
                    } else {
                        (x1 + p1.0) / 2.0
                    }
            } else {
                (p2.1 - y)
                    * if x < x1.min(p2.0) {
                        x
                    } else if x < x1.max(p2.0) {
                        x - (x - x1.min(p2.0)).powi(2) / (x1 - p2.0).abs() / 2.0
                    } else {
                        (x1 + p2.0) / 2.0
                    }
            }
        }
        (None, Some(x2)) => {
            let y = y + 1.0;
            if p1.1 < y {
                (y - p1.1)
                    * if x < x2.min(p1.0) {
                        x
                    } else if x < x2.max(p1.0) {
                        x - (x - x2.min(p1.0)).powi(2) / (x2 - p1.0).abs() / 2.0
                    } else {
                        (x2 + p1.0) / 2.0
                    }
            } else {
                (y - p2.1)
                    * if x < x2.min(p2.0) {
                        x
                    } else if x < x2.max(p2.0) {
                        x - (x - x2.min(p2.0)).powi(2) / (x2 - p2.0).abs() / 2.0
                    } else {
                        (x2 + p2.0) / 2.0
                    }
            }
        }
        (None, None) => 0.0,
    }
    .copysign(p1.1 - p2.1)
}

fn angle_norm(a1: f64, a2: f64) -> (f64, f64) {
    let (a1, a2) = if a1 < a2 { (a1, a2) } else { (a2, a1) };
    let a = a1.rem_euclid(PI * 2.0);
    (a, a2 + a - a1)
}

pub fn img_blend_pixel<X, C: PositionColor<X>>(
    buf: &mut ImageBuffer<X, Vec<u8>>,
    position_color: &C,
    x: i32,
    y: i32,
    r: f64,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    if 0 <= x && x < buf.width() as i32 && 0 <= y && y < buf.height() as i32 {
        let pixel = position_color.position_color((x, y).into());
        let (x, y) = (x as u32, y as u32);
        let pixel = blend_pixel(*buf.get_pixel(x, y), pixel, r);
        buf.put_pixel(x, y, pixel);
    }
}

pub fn blend_pixel<X>(p1: X, p2: X, r: f64) -> X
where
    X: Pixel<Subpixel = u8> + 'static,
{
    p1.map2(&p2, |a, b| (a as f64 * (1.0 - r) + b as f64 * r) as u8)
}
