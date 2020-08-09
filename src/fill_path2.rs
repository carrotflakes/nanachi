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
                    } => arc_area(*center, *radius, *angle1, *angle2, y as f64, x as f64 + 1.0) - arc_area(*center, *radius, *angle1, *angle2, y as f64, x as f64),
                })
                .sum();
            img_blend_pixel(img, position_color, x as i32, y as i32, r);//.min(1.0).max(0.0)
            //img_blend_pixel(img, position_color, x as i32, y as i32, r.min(1.0).max(0.0));
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

fn arc_area(center: Point, radius: f64, angle1: f64, angle2: f64, y: f64, x: f64) -> f64 {
    let (a1, a2) = angle_norm(angle1, angle2);
    let upper = y.max(center.1 - radius);
    let lower = (y + 1.0).min(center.1 + radius);
    if upper >= lower {
        return 0.0;
    }
    ({ // right
        match
            match (((a1 / FRAC_PI_2) as usize + 1) / 2, ((a2 / FRAC_PI_2) as usize + 1) / 2) {
                (0, 0) | (2, 2) => Some((-a2.sin(), -a1.sin(), false)),
                (0, 1) | (2, 3) => Some((-1.0, -a1.sin(), false)),
                (0, 2) | (2, 4) => Some((-a2.sin(), -a1.sin(), true)),
                (1, 1) => None,
                (1, 2) => Some((-a2.sin(), 1.0, false)),
                (1, 3) => Some((-1.0, 1.0, false)),
                _ => unreachable!(),
            } {
                Some((y1, y2, false)) => {
                    let u = upper.max(center.1 + y1 * radius);
                    let l = lower.min(center.1 + y2 * radius);
                    if u >= l { 0.0 } else {
                        right_arc_area(center, radius, u, l, x)
                    }
                }
                Some((y1, y2, true)) => {
                    0.0 + {
                        let u = upper.max(center.1 - radius);
                        let l = lower.min(center.1 + y2 * radius);
                        if u >= l { 0.0 } else {
                            right_arc_area(center, radius, u, l, x)
                        }
                    } + {
                        let u = upper.max(center.1 + y1 * radius);
                        let l = lower.min(center.1 + radius);
                        if u >= l { 0.0 } else {
                            right_arc_area(center, radius, u, l, x)
                        }
                    }
                }
                None => 0.0
            }
        }
    - { // left
        match
            match (((a1 / FRAC_PI_2) as usize + 1) / 2, ((a2 / FRAC_PI_2) as usize + 1) / 2) {
                (0, 0) | (2, 2) => None,
                (0, 1) | (2, 3) => Some((-1.0, -a2.sin(), false)),
                (0, 2) | (2, 4) => Some((-1.0, 1.0, false)),
                (1, 1) => Some((-a1.sin(), -a2.sin(), false)),
                (1, 2) => Some((-a1.sin(), 1.0, false)),
                (1, 3) => Some((-a2.sin(), -a1.sin(), true)),
                _ => unreachable!(),
            } {
                Some((y1, y2, false)) => {
                    let u = upper.max(center.1 + y1 * radius);
                    let l = lower.min(center.1 + y2 * radius);
                    if u >= l { 0.0 } else {
                    left_arc_area(center, radius, u, l, x)
                    }
                }
                Some((y1, y2, true)) => {
                    0.0 + {
                        let u = upper.max(center.1 - radius);
                        let l = lower.min(center.1 + y1 * radius);
                        if u >= l { 0.0 } else {
                            left_arc_area(center, radius, u, l, x)
                        }
                    } + {
                        let u = upper.max(center.1 + y2 * radius);
                        let l = lower.min(center.1 + radius);
                        if u >= l { 0.0 } else {
                            left_arc_area(center, radius, u, l, x)
                        }
                    }
                }
                None => 0.0
            }
    }) * (angle2 - angle1).signum()
}

#[test]
fn arc_area_test() {
    assert_eq!(arc_area(Point(10.0, 10.0), 5.0, 0.0, PI*2.0, 0.0, 10.0), 0.0);
    assert_eq!(arc_area(Point(10.0, 10.0), 5.0, 0.0, PI*2.0, 10.0, 5.0), -5.0);
    assert_eq!(arc_area(Point(10.0, 10.0), 5.0, 0.0, PI*2.0, 10.0, 6.0), -6.0);
}

fn left_arc_area(center: Point, radius: f64, upper: f64, lower: f64, x: f64) -> f64 {
    if x < center.0 - radius {
        x * (lower - upper)
    } else if x < center.0 {
        x * (lower - upper) - ((cliped_arc_area(center, radius, upper, x) - cliped_arc_area(center, radius, lower, x)))
    } else {
        center.0 * (lower - upper) - (cliped_arc_area(center, radius, upper, center.0) - cliped_arc_area(center, radius, lower, center.0))
    }
}

#[test]
fn left_arc_area_test() {
    assert_eq!(left_arc_area(Point(10.0, 10.0), 5.0, 10.0, 11.0, 5.0), 5.0);
    assert_eq!(left_arc_area(Point(10.0, 10.0), 5.0, 10.0, 11.0, 6.0), 5.0);
}

fn right_arc_area(center: Point, radius: f64, upper: f64, lower: f64, x: f64) -> f64 {
    if x < center.0 {
        x * (lower - upper)
    } else if x < center.0 + radius {
        center.0 * (lower - upper) + (cliped_arc_area(center, radius, upper, x) - cliped_arc_area(center, radius, lower, x)) - (cliped_arc_area(center, radius, upper, center.0) - cliped_arc_area(center, radius, lower, center.0))
    } else {
        center.0 * (lower - upper) + (cliped_arc_area(center, radius, upper, center.0) - cliped_arc_area(center, radius, lower, center.0))
    }
}

fn cliped_arc_area(center: Point, radius: f64, upper: f64, right: f64) -> f64 {
    fn f(d: f64) -> f64 {
        d.acos() - (1.0 - d * d).sqrt() * d
    }
    let w = (center.0 - right) / radius;
    let h = (upper - center.1) / radius;

    radius.powi(2) *
    if w.powi(2) + h.powi(2) >= 1.0 {
        match (0.0 <= w, 0.0 <= h) {
            (true, true) => 0.0,
            (true, false) => f(w),
            (false, true) => f(h),
            (false, false) => PI - f(-w) - f(-h),
        }
    } else {
        match (0.0 <= w, 0.0 <= h) {
            (true, true) => -1.0 * PI / 4.0 + f(w) / 2.0 + f(h) / 2.0 + w * h,
            (true, false) => 1.0 * PI / 4.0 + f(w) / 2.0 - f(-h) / 2.0 + w * h,
            (false, true) => 1.0 * PI / 4.0 - f(-w) / 2.0 + f(h) / 2.0 + w * h,
            (false, false) => 3.0 * PI / 4.0 - f(-w) / 2.0 - f(-h) / 2.0 + w * h,
        }
    }
}

#[test]
fn cliped_arc_area_test() {
    assert_eq!(cliped_arc_area(Point(0.0, 0.0), 1.0, 0.0, 0.0), PI / 4.0);
    assert_eq!(cliped_arc_area(Point(0.0, 0.0), 1.0, 0.5, 0.0), (0.5f64.acos() - (1.0f64 - 0.5 * 0.5).sqrt() * 0.5) / 2.0);
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
        (None, None) => 0.0, // TODO この間に存在するパターンがある
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
