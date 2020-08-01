use crate::geometry;
use crate::point::Point;
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

pub fn draw_path<P: Into<Point> + Copy>(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, ps: &[P], pixel: Rgb<u8>) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            for pair in ps.windows(2) {
                let d = geometry::distance_between_line_segment_and_point(
                    pair[0],
                    pair[1],
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

pub fn draw_fill(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, ps: &[(f64, f64)], pixel: Rgb<u8>) {
    for y in 0..img.height() {
        let mut vec = ps
            .windows(2)
            .filter_map(|pair| {
                geometry::intersect_segment_and_horizon(
                    pair[0].0, pair[0].1, pair[1].0, pair[1].1, y as f64,
                )
            })
            .collect::<Vec<f64>>();
        vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..vec.len() / 2 {
            let s = vec[i * 2].max(0.0) as u32;
            let e = vec[i * 2 + 1].max(0.0).min(img.width() as f64) as u32;
            for x in s..e {
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
