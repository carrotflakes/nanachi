use nanachi::{
    legacy_draw as draw,
    image::{ImageBuffer, Rgb},
    fill_color,
};
use rand_core::RngCore;
use rand_pcg::Pcg32;
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        if (x / 8 + y / 8) % 2 == 0 {
            Rgb([240u8, 240, 240])
        } else {
            Rgb([255, 255, 255])
        }
    });

    draw_stars(&mut img);

    draw_nanachi(&mut img);

    let res = img.save("./legacy_nanachi.png");
    println!("{:?}", res);
}

fn draw_stars(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let spoke = (2.0 * PI / 5.0).cos() / (1.0 * PI / 5.0).cos();
    let points = (0..=10)
        .map(|i| {
            let p = i as f64 / 10.0 * PI * 2.0;
            let (s, c) = p.sin_cos();
            let r = (1.0 - (i % 2) as f64 * (1.0 - spoke)) * 10.0;
            (s * r, c * r)
        })
        .collect::<Vec<_>>();

    let mut rnd = Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
    let shapes = (0..100)
        .map(|_| {
            let t = (
                rnd.next_u32() as f64 / std::u32::MAX as f64 * img.width() as f64,
                rnd.next_u32() as f64 / std::u32::MAX as f64 * img.height() as f64,
            );
            let r = rnd.next_u32() as f64 / std::u32::MAX as f64 * PI * 2.0;
            let s = rnd.next_u32() as f64 / std::u32::MAX as f64 * 4.0 + 5.0;
            points
                .iter()
                .map(|p| transform(p, t, r, (s, s)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (si, ps) in shapes.iter().enumerate() {
        let pc = fill_color::Solid::new(Rgb(
            [[255, 128, 0], [0, 255, 128], [128, 0, 255]][si % 3]
        ));
        draw::draw_fill(img, &vec![ps], &pc);
        for s in ps.windows(2) {
            draw::draw_line(
                img,
                s[0],
                s[1],
                Rgb([[128, 64, 0], [0, 128, 64], [64, 0, 128]][si % 3]),
            );
        }
    }
}

fn draw_nanachi(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
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
        vec![(0.30, 0.51), (0.40, 0.50)],
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
        vec![(0.60, 0.50), (0.70, 0.51)],
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
    ]
    .iter()
    .map(|v| {
        v.iter()
            .map(|p| (p.0 * 512.0, p.1 * 512.0))
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    let mut shape = Vec::new();
    let f = |l: (f64, f64), c: (f64, f64), r: (f64, f64)| {
        let ll = (l.0 - c.0).atan2(l.1 - c.1);
        let rr = (c.0 - r.0).atan2(c.1 - r.1);
        (
            c.0 - (ll.cos() + rr.cos()) * 8.0,
            c.1 + (ll.sin() + rr.sin()) * 8.0,
        )
    };
    shape.push(f(
        nanachi[0][nanachi[0].len() - 1],
        nanachi[0][0],
        nanachi[0][1],
    ));
    for i in 0..nanachi[0].len() - 2 {
        shape.push(f(nanachi[0][i], nanachi[0][i + 1], nanachi[0][i + 2]));
    }
    shape.push(f(
        nanachi[0][nanachi[0].len() - 2],
        nanachi[0][nanachi[0].len() - 1],
        nanachi[0][0],
    ));
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
    ]
    .iter()
    .map(|p| (p.0 * 512.0, p.1 * 512.0))
    .collect::<Vec<_>>();

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
        vec![(0.24, 0.80), (0.25, 0.81)],
        vec![
            (0.58, 0.76),
            (0.56, 0.86),
            (0.44, 0.85),
            (0.42, 0.82),
            (0.53, 0.80),
            (0.65, 0.79),
        ],
        vec![(0.52, 0.82), (0.53, 0.83)],
        vec![
            (0.83, 0.78),
            (0.66, 0.81),
            (0.72, 0.73),
            (0.74, 0.80),
            (0.78, 0.87),
        ],
    ]
    .iter()
    .map(|v| {
        v.iter()
            .map(|p| (p.0 * 512.0, p.1 * 512.0))
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    let pc = fill_color::Solid::new(Rgb([255, 235, 230]));

    draw::draw_fill(img, &vec![&shape], &pc);

    draw::draw_fill(img, &vec![&moji_shape], &pc);

    for ps in nanachi.iter() {
        for s in ps.windows(2) {
            draw::draw_line(img, s[0], s[1], Rgb([64, 8, 8]));
        }
        //draw::draw_path(img, ps, Rgb([64, 8, 8]));
    }

    for ps in moji.iter() {
        for s in ps.windows(2) {
            draw::draw_line(img, s[0], s[1], Rgb([64, 8, 8]));
        }
    }
}

 fn transform(
    p: &(f64, f64),
    translation: (f64, f64),
    rotation: f64,
    scale: (f64, f64),
) -> (f64, f64) {
    let (x, y) = p;
    let (sin, cos) = rotation.sin_cos();
    let (x, y) = (x * cos - y * sin, x * sin + y * cos);
    let (x, y) = (x * scale.0, y * scale.1);
    let (x, y) = (x + translation.0, y + translation.1);
    (x, y)
}
