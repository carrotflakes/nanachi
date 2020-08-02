use crate::geometry;
use crate::{point::Point, position_color::PositionColor};
use image::{ImageBuffer, Rgb};

pub fn draw_line<P: Into<Point>>(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    p1: P,
    p2: P,
    pixel: Rgb<u8>,
) {
    let mut p1: Point = p1.into();
    let mut p2: Point = p2.into();
    if (p1.0 - p2.0).abs() < (p1.1 - p2.1).abs() {
        if p1.1 > p2.1 {
            std::mem::swap(&mut p1, &mut p2);
        }
        for y in p1.1.max(0.0) as u32..=p2.1.max(0.0).min(img.height() as f64 - 1.0) as u32 {
            let x = (p2.0 - p1.0) * ((y as f64 - p1.1) / (p2.1 - p1.1)) + p1.0;
            if 0.0 <= x && x < img.width() as f64 {
                img.put_pixel(x as u32, y, pixel);
            }
        }
    } else {
        if p1.0 > p2.0 {
            std::mem::swap(&mut p1, &mut p2);
        }
        for x in p1.0.max(0.0) as u32..=p2.0.max(0.0).min(img.width() as f64 - 1.0) as u32 {
            let y = (p2.1 - p1.1) * ((x as f64 - p1.0) / (p2.0 - p1.0)) + p1.1;
            if 0.0 <= y && y < img.height() as f64 {
                img.put_pixel(x, y as u32, pixel);
            }
        }
    }
}

pub fn draw_path<P: Into<Point> + Copy>(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ps: &[P],
    pixel: Rgb<u8>,
    stroke_width: f64,
) {
    const GRAD_WIDTH: f64 = 1.5;
    for y in 0..img.height() {
        for x in 0..img.width() {
            let mut alpha = 0.0;
            for pair in ps.windows(2) {
                let d = geometry::distance_between_line_segment_and_point(
                    pair[0],
                    pair[1],
                    (x as f64, y as f64),
                );
                if d < stroke_width {
                    img.put_pixel(x, y, pixel);
                } else if d < stroke_width + GRAD_WIDTH {
                    let r = (stroke_width + GRAD_WIDTH - d) / GRAD_WIDTH;
                    let r = 1.0 - r * 2.0;
                    let a = (r.acos() - (1.0 - r.powi(2)).sqrt()) / std::f64::consts::PI; // (acos(x) - sqrt(1-x^2)*x) / PI
                    if alpha < a {
                        alpha = a;
                    }
                }
            }
            if 0.0 < alpha {
                img.put_pixel(
                    x,
                    y,
                    blend_rgb(
                        *img.get_pixel(x, y),
                        pixel,
                        alpha,
                    ),
                );
            }
        }
    }
}

pub fn draw_lines<P: Into<Point> + Copy>(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    lines: Vec<(P, P)>,
    pixel: Rgb<u8>,
) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            for pair in lines.iter() {
                let d = geometry::distance_between_line_segment_and_point(
                    pair.0,
                    pair.1,
                    (x as f64, y as f64),
                );
                if d < 5.0 {
                    img.put_pixel(x, y, pixel);
                } else if d < 6.0 {
                    img.put_pixel(x, y, blend_rgb(*img.get_pixel(x, y), pixel, 6.0 - d));
                }
            }
        }
    }
}

pub fn draw_fill<P: Into<Point> + Copy, C: PositionColor<Rgb<u8>>>(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ps: &[P],
    position_color: &C,
) {
    for y in 0..img.height() {
        let mut vec = ps
            .windows(2)
            .filter_map(|pair| {
                let p1: Point = pair[0].into();
                let p2: Point = pair[1].into();
                geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y as f64)
            })
            .collect::<Vec<f64>>();
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

pub fn blend_rgb(p1: Rgb<u8>, p2: Rgb<u8>, r: f64) -> Rgb<u8> {
    Rgb([
        (p1[0] as f64 * (1.0 - r) + p2[0] as f64 * r) as u8,
        (p1[1] as f64 * (1.0 - r) + p2[1] as f64 * r) as u8,
        (p1[2] as f64 * (1.0 - r) + p2[2] as f64 * r) as u8,
    ])
}
