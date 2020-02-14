extern crate image;
extern crate rand_pcg;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
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
    println!("{:?}", rnd.next_u32());
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

    for y in 0..512 {
        //let mut fliped = false;
        // let mut flipeds = vec![false; shapes.len()];
        // for x in 0..512 {
        //     for (i, ps) in shapes.iter().enumerate() {
        //         flipeds[i] = flipeds[i] ^ (ps.windows(2).filter(|pair| intersect(pair[0].0, pair[0].1, pair[1].0, pair[1].1, x as f64, y as f64, x as f64 + 1.0, y as f64)).count() % 2 == 1);
        //         if flipeds[i] {
        //             img.put_pixel(x, y, image::Rgb([255, 128, 0]));
        //         }
        //     }
        // }
        for (si, ps) in shapes.iter().enumerate() {
            let mut vec = ps.windows(2).filter_map(|pair| intersection_(pair[0].0, pair[0].1, pair[1].0, pair[1].1, y as f64)).collect::<Vec<f64>>();
            vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            if !vec.is_empty() {
                for i in 0..vec.len() / 2 {
                    let s = vec[i * 2].max(0.0) as u32;
                    let e = vec[i * 2 + 1].max(0.0).min((width - 1) as f64) as u32;
                    for x in s..e {
                        img.put_pixel(x, y, image::Rgb([[255, 128, 0], [0, 255, 128], [128, 0, 255]][si % 3]));
                    }
                }
            }
        }
    }

    let res = img.save("./my_image.png");
    println!("{:?}", res);
}

// fn intersect(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, dx: f64, dy: f64) -> bool {
//     let ta = (cx - dx) * (ay - cy) + (cy - dy) * (cx - ax);
//     let tb = (cx - dx) * (by - cy) + (cy - dy) * (cx - bx);
//     let tc = (ax - bx) * (cy - ay) + (ay - by) * (ax - cx);
//     let td = (ax - bx) * (dy - ay) + (ay - by) * (ax - dx);
//     tc * td < 0.0 && ta * tb <= 0.0
// }

// fn intersection(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, dx: f64, dy: f64) -> Option<(f64, f64)> {
// 	let ksi = (dy - cy) * (dx - ax) - (dx - cx) * (dy - ay);
// 	let eta = (bx - ax) * (dy - ay) - (by - ay) * (dx - ax);
// 	let delta = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);

// 	let lambda = ksi / delta;
// 	let mu = eta / delta;

// 	if lambda >= 0.0 && lambda <= 1.0 && mu >= 0.0 && mu <= 1.0 {
// 		Some((ax + lambda * (bx - ax), ay + lambda * (by - ay)))
// 	} else {
//         None
//     }
// }

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
    let (sin, cos) = (-rotation).sin_cos();
    let (x, y) = (x * cos - y * sin, x * sin + y * cos);
    let (x, y) = (x * scale.0, y * scale.1);
    let (x, y) = (x + translation.0, y + translation.1);
    (x, y)
}
