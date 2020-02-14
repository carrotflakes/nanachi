extern crate image;
extern crate rand_pcg;

use image::ImageBuffer;
use std::f64::consts::PI;
use rand_core::RngCore;
use rand_pcg::Pcg32;

fn main() {
    let (width, height) = (512, 512);

    let spoke = (2.0 * PI / 5.0).cos() / (1.0 * PI / 5.0).cos();
    let points = (0..=10).map(|i| {
        let p = i as f64 / 10.0 * PI * 2.0;
        let (s, c) = p.sin_cos();
        let r = (1.0 - (i % 2) as f64 * (1.0 - spoke)) * 10.0;
        (s * r, c * r)
    }).collect::<Vec<_>>();

    let mut rnd = Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
    let shapes = (0..100).map(|_| {
        let t = (rnd.next_u32() as f64 / std::u32::MAX as f64 * width as f64, rnd.next_u32() as f64 / std::u32::MAX as f64 * height as f64);
        let r = rnd.next_u32() as f64 / std::u32::MAX as f64 * PI * 2.0;
        let s = rnd.next_u32() as f64 / std::u32::MAX as f64 * 4.0 + 5.0;
        points.iter().map(|p| transform(p, t, r, (s, s))).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    println!("{:?}", shapes);

    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        if (x / 8 + y / 8) % 2 == 0 {
            image::Rgb([240u8, 240, 240])
        } else {
            image::Rgb([255, 255, 255])
        }
    });

    for (si, ps) in shapes.iter().enumerate() {
        for y in 0..height {
            let mut vec = ps.windows(2).filter_map(|pair| intersection_(pair[0].0, pair[0].1, pair[1].0, pair[1].1, y as f64)).collect::<Vec<f64>>();
            vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            for i in 0..vec.len() / 2 {
                let s = vec[i * 2].max(0.0) as u32;
                let e = vec[i * 2 + 1].max(0.0).min((width - 1) as f64) as u32;
                for x in s..e {
                    img.put_pixel(x, y, image::Rgb([[255, 128, 0], [0, 255, 128], [128, 0, 255]][si % 3]));
                }
            }
        }
        for s in ps.windows(2) {
            draw_line(&mut img, s[0], s[1], image::Rgb([[128, 64, 0], [0, 128, 64], [64, 0, 128]][si % 3]));
        }
    }

    let res = img.save("./my_image.png");
    println!("{:?}", res);
}

fn intersection_(ax: f64, ay: f64, bx: f64, by: f64, hy: f64) -> Option<f64> {
    if ay != by && ((hy < ay) ^ (hy < by)) {
        let r = (hy - ay) / (by - ay);
        Some(ax * (1.0 - r) + bx * r)
    } else {
        None
    }
}

fn transform(p: &(f64, f64), translation: (f64, f64), rotation: f64, scale: (f64, f64)) -> (f64, f64) {
    let (x, y) = p;
    let (sin, cos) = rotation.sin_cos();
    let (x, y) = (x * cos - y * sin, x * sin + y * cos);
    let (x, y) = (x * scale.0, y * scale.1);
    let (x, y) = (x + translation.0, y + translation.1);
    (x, y)
}

fn draw_line(img: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, mut p1: (f64, f64), mut p2: (f64, f64), pixel: image::Rgb<u8>) {
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
