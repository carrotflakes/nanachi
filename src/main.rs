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

    //println!("{:?}", shapes);

    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        if (x / 8 + y / 8) % 2 == 0 {
            image::Rgb([240u8, 240, 240])
        } else {
            image::Rgb([255, 255, 255])
        }
    });

    for (si, ps) in shapes.iter().enumerate() {
        draw_fill(&mut img, ps.as_slice(), image::Rgb([[255, 128, 0], [0, 255, 128], [128, 0, 255]][si % 3]));
        for s in ps.windows(2) {
            draw_line(&mut img, s[0], s[1], image::Rgb([[128, 64, 0], [0, 128, 64], [64, 0, 128]][si % 3]));
        }
    }

    draw_nanachi(&mut img);

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

fn draw_fill(img: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, ps: &[(f64, f64)], pixel: image::Rgb<u8>) {
    for y in 0..img.height() {
        let mut vec = ps.windows(2).filter_map(|pair| intersection_(pair[0].0, pair[0].1, pair[1].0, pair[1].1, y as f64)).collect::<Vec<f64>>();
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

fn draw_nanachi(img: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    let nanachi = vec![
        // contour
        vec![
            (0.41, 0.75),
            (0.30, 0.74),
            (0.20, 0.69),
            (0.18, 0.60),
            (0.20, 0.52),
            (0.24, 0.45),
            (0.33, 0.40),
            (0.30, 0.25),
            (0.33, 0.12),
            (0.36, 0.08),
            (0.40, 0.15),
            (0.39, 0.28),
            (0.38, 0.40),
            (0.48, 0.38),
            (0.57, 0.37),
            (0.57, 0.25),
            (0.62, 0.10),
            (0.67, 0.05),
            (0.70, 0.10),
            (0.67, 0.25),
            (0.62, 0.38),
            (0.74, 0.42),
            (0.80, 0.50),
            (0.82, 0.60),
            (0.78, 0.70),
            (0.65, 0.76),
            (0.52, 0.76),
        ],
        // left eyelid
        vec![
            (0.30, 0.51),
            (0.40, 0.50),
        ],
        // left eye
        vec![
            (0.33, 0.51),
            (0.32, 0.55),
            (0.33, 0.61),
            (0.37, 0.61),
            (0.38, 0.55),
            (0.37, 0.50),
        ],
        // right eyelid
        vec![
            (0.60, 0.50),
            (0.70, 0.51),
        ],
        // right
        vec![
            (0.63, 0.50),
            (0.62, 0.55),
            (0.63, 0.61),
            (0.67, 0.61),
            (0.68, 0.55),
            (0.67, 0.51),
        ],
        // upper lip
        vec![
            (0.40, 0.66),
            (0.45, 0.67),
            (0.50, 0.66),
            (0.55, 0.67),
            (0.60, 0.66),
        ],
        // lower lip
        vec![
            (0.45, 0.67),
            (0.46, 0.70),
            (0.50, 0.71),
            (0.54, 0.70),
            (0.55, 0.67),
        ],
    ].iter().map(|v| v.iter().map(|p| (p.0 * 512.0, p.1 * 512.0)).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut shape = Vec::new();
    let f = |l: (f64, f64), c: (f64, f64), r: (f64, f64)| {
        let ll = (l.0 - c.0).atan2(l.1 - c.1);
        let rr = (c.0 - r.0).atan2(c.1 - r.1);
        (c.0 - (ll.cos() + rr.cos()) * 8.0, c.1 + (ll.sin() + rr.sin()) * 8.0)
    };
    shape.push(f(nanachi[0][nanachi[0].len()-1], nanachi[0][0], nanachi[0][1]));
    for i in 0..nanachi[0].len()-2 {
        shape.push(f(nanachi[0][i], nanachi[0][i+1], nanachi[0][i+2]));
    }
    shape.push(f(nanachi[0][nanachi[0].len()-2], nanachi[0][nanachi[0].len()-1], nanachi[0][0]));
    shape.push(shape[0]);

    let moji_shape = vec![
        (0.30, 0.73),
        (0.20, 0.75),
        (0.15, 0.78),
        (0.15, 0.86),
        (0.30, 0.88),
        (0.50, 0.89),
        (0.67, 0.87),
        (0.80, 0.90),
        (0.85, 0.83),
        (0.86, 0.74),
        (0.30, 0.74),
    ].iter().map(|p| (p.0 * 512.0, p.1 * 512.0)).collect::<Vec<_>>();

    let moji = vec![
        vec![
            (0.30, 0.74),
            (0.30, 0.86),
            (0.20, 0.85),
            (0.17, 0.80),
            (0.21, 0.77),
            (0.30, 0.76),
            (0.42, 0.77),
        ],
        vec![
            (0.24, 0.80),
            (0.25, 0.81),
        ],
        vec![
            (0.58, 0.76),
            (0.56, 0.86),
            (0.44, 0.85),
            (0.42, 0.82),
            (0.53, 0.80),
            (0.65, 0.79),
        ],
        vec![
            (0.52, 0.82),
            (0.53, 0.83),
        ],
        vec![
            (0.83, 0.78),
            (0.66, 0.81),
            (0.72, 0.73),
            (0.74, 0.80),
            (0.78, 0.87),
        ]
    ].iter().map(|v| v.iter().map(|p| (p.0 * 512.0, p.1 * 512.0)).collect::<Vec<_>>()).collect::<Vec<_>>();

    draw_fill(img, shape.as_slice(), image::Rgb([255, 235, 230]));

    draw_fill(img, moji_shape.as_slice(), image::Rgb([255, 235, 230]));

    for ps in nanachi.iter() {
        for s in ps.windows(2) {
            draw_line(img, s[0], s[1], image::Rgb([64, 8, 8]));
        }
    }

    for ps in moji.iter() {
        for s in ps.windows(2) {
            draw_line(img, s[0], s[1], image::Rgb([64, 8, 8]));
        }
    }
}
