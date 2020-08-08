use crate::geometry;
use crate::path2::PathEdge;
use crate::position_color::PositionColor;
use image::{ImageBuffer, Pixel};
use std::f64::consts::{FRAC_PI_2, PI};

pub fn draw_fill<X, C: PositionColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    edges: &Vec<PathEdge>,
    position_color: &C,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    for y in 0..img.height() {
        let mut vec = scan(edges, y as f64);
        vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..vec.len() / 2 {
            let s = vec[i * 2].max(0.0) as u32;
            let e = vec[i * 2 + 1].max(0.0).min(img.width() as f64) as u32;
            for x in s..e {
                let pixel = position_color.position_color((x, y).into());
                img.put_pixel(x, y, pixel);
            }
        }
    }
}

fn scan(edges: &Vec<PathEdge>, y: f64) -> Vec<f64>{
    let mut vec = Vec::new();
    for edge in edges {
        match edge {
            PathEdge::Line(p1, p2) => {
                if let Some(x) =
                    geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y)
                {
                    vec.push(x);
                }
            }
            PathEdge::Arc {
                center,
                radius,
                angle1,
                angle2,
            } => {
                let y = y - center.1;
                if -radius <= y && y < *radius {
                    let (angle1, angle2) = angle_norm(*angle1, *angle2);
                    let a = -(y / radius).asin();
                    let d = a.cos() * radius;
                    if (angle1 <= a && a < angle2)
                        || (angle1 <= a + PI * 2.0 && a + PI * 2.0 < angle2)
                    {
                        vec.push(center.0 + d);
                    }
                    let a = PI - (a + FRAC_PI_2) + FRAC_PI_2;
                    if (angle1 <= a && a < angle2)
                        || (angle1 <= a + PI * 2.0 && a + PI * 2.0 < angle2)
                    {
                        vec.push(center.0 - d);
                    }
                }
            }
        }
    }
    vec
}

fn angle_norm(a1: f64, a2: f64) -> (f64, f64) {
    let (a1, a2) = if a1 < a2 { (a1, a2) } else { (a2, a1) };
    let a = a1.rem_euclid(PI * 2.0);
    (a, a2 + a - a1)
}
