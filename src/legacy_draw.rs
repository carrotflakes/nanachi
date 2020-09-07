use crate::geometry;
use crate::{point::Point, fill_color::FillColor};
use image::{ImageBuffer, Pixel, Rgb};

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
    const GRAD_WIDTH: f64 = 1.0;
    for y in 0..img.height() {
        for x in 0..img.width() {
            let mut alpha = 0.0;
            for pair in ps.windows(2) {
                let d = geometry::squared_distance_between_line_segment_and_point(
                    pair[0],
                    pair[1],
                    (x as f64, y as f64),
                )
                .sqrt();
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
                img.put_pixel(x, y, blend_rgb(*img.get_pixel(x, y), pixel, alpha));
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
                let d = geometry::squared_distance_between_line_segment_and_point(
                    pair.0,
                    pair.1,
                    (x as f64, y as f64),
                )
                .sqrt();
                if d < 5.0 {
                    img.put_pixel(x, y, pixel);
                } else if d < 6.0 {
                    img.put_pixel(x, y, blend_rgb(*img.get_pixel(x, y), pixel, 6.0 - d));
                }
            }
        }
    }
}

pub fn draw_fill<X, P: Into<Point> + Copy, C: FillColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    pss: &Vec<&Vec<P>>,
    fill_color: &C,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    for y in 0..img.height() {
        let mut vec = pss
            .iter()
            .flat_map(|ps| {
                ps.windows(2).filter_map(|pair| {
                    let p1: Point = pair[0].into();
                    let p2: Point = pair[1].into();
                    geometry::intersect_segment_and_horizon(p1.0, p1.1, p2.0, p2.1, y as f64)
                })
            })
            .collect::<Vec<f64>>();
        vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..vec.len() / 2 {
            let s = vec[i * 2].max(0.0) as u32;
            let e = vec[i * 2 + 1].max(0.0).min(img.width() as f64) as u32;
            for x in s..e {
                let pixel = fill_color.fill_color(x as f64, y as f64);
                img.put_pixel(x, y, pixel);
            }
        }
    }
}

pub fn draw_hori<P: Into<Point> + Copy, C: FillColor<Rgb<u8>>>(
    buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    center: P,
    rotate: f64,
    fill_color: &C,
) {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    let r = rotate.rem_euclid(2.0 * PI) + FRAC_PI_4;
    let center: Point = center.into();
    if r <= PI {
        if r <= FRAC_PI_2 {
            // down
            for x in 0..buf.width() {
                let yy = center.1 + rotate.sin() * (x as f64 - center.0);
                for y in yy.round().max(0.0) as u32..buf.height() {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x, y, pixel);
                }
            }
        } else {
            // left
            for y in 0..buf.height() {
                let xx = center.0 + rotate.cos() * (y as f64 - center.1);
                for x in 0..(xx.round().max(0.0) as u32).min(buf.width()) {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x, y, pixel);
                }
            }
        }
    } else {
        if r <= PI + FRAC_PI_2 {
            // up
            for x in 0..buf.width() {
                let yy = center.1 - rotate.sin() * (x as f64 - center.0);
                for y in 0..(yy.round().max(0.0) as u32).min(buf.height()) {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x, y, pixel);
                }
            }
        } else {
            // right
            for y in 0..buf.height() {
                let xx = center.0 - rotate.cos() * (y as f64 - center.1);
                for x in xx.round().max(0.0) as u32..buf.width() {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x, y, pixel);
                }
            }
        }
    }
}

pub fn draw_hori_with_antialias<P: Into<Point> + Copy, C: FillColor<Rgb<u8>>>(
    buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    center: P,
    rotate: f64,
    fill_color: &C,
) {
    use std::f64::consts::FRAC_PI_4;
    let center: Point = center.into();
    fn floor_fract(f: f64) -> (i32, f64) {
        (f.floor() as i32, f.fract())
    }
    let mix = |buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, alpha: f64| {
        if x < 0 || y < 0 || buf.width() as i32 <= x || buf.height() as i32 <= y {
            return;
        }
        let pixel = fill_color.fill_color(x as f64, y as f64);
        buf.put_pixel(
            x as u32,
            y as u32,
            blend_rgb(*buf.get_pixel(x as u32, y as u32), pixel, alpha),
        );
    };
    let (sin, cos) = rotate.sin_cos();
    match ((rotate / FRAC_PI_4) as i32).rem_euclid(8) {
        0 => {
            let mut a = floor_fract(center.1 + sin * -center.0);
            for x in 0..buf.width() as i32 {
                let b = floor_fract(center.1 + sin * ((x + 1) as f64 - center.0));
                if a.0 == b.0 {
                    mix(buf, x, a.0, 1.0 - (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, x, a.0, (1.0 - a.1).powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                    mix(buf, x, b.0, 1.0 - b.1.powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                }
                for y in (b.0 + 1).max(0)..buf.height() as i32 {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        1 => {
            let mut a = floor_fract(center.0 + cos * -center.1);
            for y in 0..buf.height() as i32 {
                let b = floor_fract(center.0 + cos * ((y + 1) as f64 - center.1));
                if a.0 == b.0 {
                    mix(buf, a.0, y, (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, b.0, y, b.1.powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                    mix(
                        buf,
                        a.0,
                        y,
                        1.0 - (1.0 - a.1).powi(2) * (b.1 + 1.0 - a.1) / 2.0,
                    );
                }
                for x in 0..a.0.min(buf.width() as i32) {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        2 => {
            let mut a = floor_fract(center.0 + cos * -center.1);
            for y in 0..buf.height() as i32 {
                let b = floor_fract(center.0 + cos * ((y + 1) as f64 - center.1));
                if a.0 == b.0 {
                    mix(buf, a.0, y, (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, a.0, y, b.1.powi(2) * (a.1 + 1.0 - b.1) / 2.0);
                    mix(
                        buf,
                        b.0,
                        y,
                        1.0 - (1.0 - a.1).powi(2) * (a.1 + 1.0 - b.1) / 2.0,
                    );
                }
                for x in 0..a.0.min(buf.width() as i32) {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        3 => {
            let mut a = floor_fract(center.1 - sin * -center.0);
            for x in 0..buf.width() as i32 {
                let b = floor_fract(center.1 - sin * ((x + 1) as f64 - center.0));
                if a.0 == b.0 {
                    mix(buf, x, a.0, (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, x, a.0, (1.0 - a.1).powi(2) * (a.1 + 1.0 - b.1) / 2.0);
                    mix(buf, x, b.0, 1.0 - b.1.powi(2) * (a.1 + 1.0 - b.1) / 2.0);
                }
                for y in 0..a.0.min(buf.height() as i32) {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        4 => {
            let mut a = floor_fract(center.1 - sin * -center.0);
            for x in 0..buf.width() as i32 {
                let b = floor_fract(center.1 - sin * ((x + 1) as f64 - center.0));
                if a.0 == b.0 {
                    mix(buf, x, a.0, (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, x, b.0, (1.0 - b.1).powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                    mix(buf, x, a.0, 1.0 - a.1.powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                }
                for y in 0..a.0.min(buf.height() as i32) {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        5 => {
            let mut a = floor_fract(center.0 - cos * -center.1);
            for y in 0..buf.height() as i32 {
                let b = floor_fract(center.0 - cos * ((y + 1) as f64 - center.1));
                if a.0 == b.0 {
                    mix(buf, a.0, y, 1.0 - (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, a.0, y, (1.0 - a.1).powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                    mix(buf, b.0, y, 1.0 - b.1.powi(2) * (b.1 + 1.0 - a.1) / 2.0);
                }
                for x in (b.0 + 1).max(0)..buf.width() as i32 {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        6 => {
            let mut a = floor_fract(center.0 - cos * -center.1);
            for y in 0..buf.height() as i32 {
                let b = floor_fract(center.0 - cos * ((y + 1) as f64 - center.1));
                if a.0 == b.0 {
                    mix(buf, a.0, y, 1.0 - (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, b.0, y, (1.0 - b.1).powi(2) * (a.1 + 1.0 - b.1) / 2.0);
                    mix(buf, a.0, y, 1.0 - a.1.powi(2) * (a.1 + 1.0 - b.1) / 2.0);
                }
                for x in (b.0 + 1).max(0)..buf.width() as i32 {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        7 => {
            let mut a = floor_fract(center.1 + sin * -center.0);
            for x in 0..buf.width() as i32 {
                let b = floor_fract(center.1 + sin * ((x + 1) as f64 - center.0));
                if a.0 == b.0 {
                    mix(buf, x, a.0, 1.0 - (a.1 + b.1) / 2.0);
                } else {
                    mix(buf, x, b.0, (1.0 - b.1).powi(2) / 2.0 * (a.1 + 1.0 - b.1));
                    mix(buf, x, a.0, 1.0 - a.1.powi(2) / 2.0 * (a.1 + 1.0 - b.1));
                }
                for y in (a.0 + 1).max(0)..buf.height() as i32 {
                    let pixel = fill_color.fill_color(x as f64, y as f64);
                    buf.put_pixel(x as u32, y as u32, pixel);
                }
                a = b;
            }
        }
        _ => unreachable!(),
    }
}

pub fn blend_rgb(p1: Rgb<u8>, p2: Rgb<u8>, r: f64) -> Rgb<u8> {
    Rgb([
        (p1[0] as f64 * (1.0 - r) + p2[0] as f64 * r) as u8,
        (p1[1] as f64 * (1.0 - r) + p2[1] as f64 * r) as u8,
        (p1[2] as f64 * (1.0 - r) + p2[2] as f64 * r) as u8,
    ])
}
