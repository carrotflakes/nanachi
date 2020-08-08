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
        vec.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for i in 0..vec.len() / 2 {
            let s = vec[i * 2].0.max(0.0) as u32;
            let e = vec[i * 2 + 1].0.max(0.0).min(img.width() as f64) as u32;
            for x in s..e {
                img_blend_pixel(img, position_color, x as i32, y as i32, 1.0);
            }
        }
    }
}

fn scan(edges: &Vec<PathEdge>, y: f64) -> Vec<(f64, usize)> {
    let mut vec = Vec::new();
    for (i, edge) in edges.iter().enumerate() {
        match edge {
            PathEdge::Line(p1, p2) => {
                if let Some(x) = geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y)
                {
                    vec.push((x, i));
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
                        vec.push((center.0 + d, i));
                    }
                    let a = PI - (a + FRAC_PI_2) + FRAC_PI_2;
                    if (angle1 <= a && a < angle2)
                        || (angle1 <= a + PI * 2.0 && a + PI * 2.0 < angle2)
                    {
                        vec.push((center.0 - d, i));
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
