use nanachi::{
    buffer::GenericBuffer,
    compositor,
    context::{ChildContext, Context, FillStyle},
    fill_color, fill_rule,
    image::RgbaImage,
    contrib::k_curve::k_curve,
    matrix::Matrix,
    path::Path,
    path_builder::PathBuilder,
    path_data_notation,
    path_transform::path_transform,
    pixel::{PremultipliedRgba, Rgba},
};
use rand_core::RngCore;
use rand_pcg::Pcg32;
use std::f64::consts::PI;

type Pixel = Rgba;
// type Pixel = PremultipliedRgba;

fn main() {
    let (width, height) = (512, 512);
    let mut context = Context::from_pixel(width, height, rgba(255, 255, 255, 255)).high_quality();
    context.clear(&fill_color::LinearGradient::new(
        (0.0, 0.0),
        (0.0, height as f64),
        vec![
            (0.0, rgba(255, 255, 255, 255)),
            (1.0, rgba(130, 130, 130, 255)),
        ],
    ));

    let t = std::time::Instant::now();
    draw_stars(context.child());
    println!("elapsed: {:?}", t.elapsed());
    let t = std::time::Instant::now();
    draw_nanachi(context.child());
    println!("elapsed: {:?}", t.elapsed());
    draw_frame(context.child());

    let img: RgbaImage = (&context.image).into();
    let res = img.save("./nanachi.png");
    println!("{:?}", res);
}

fn draw_stars<'a>(mut context: ChildContext<'a, Pixel, GenericBuffer<Pixel>>) {
    let spoke = (2.0 * PI / 5.0).cos() / (1.0 * PI / 5.0).cos();
    let mut pb = PathBuilder::new();
    for i in 0..10 {
        let p = i as f64 / 10.0 * PI * 2.0;
        let (s, c) = p.sin_cos();
        let r = (1.0 - (i % 2) as f64 * (1.0 - spoke)) * 10.0;
        pb.line_to(-s * r, c * r);
    }
    pb.close();
    let path = pb.end();

    let mut rnd = Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);

    for i in 0..100 {
        let t = (
            rnd.next_u32() as f64 / std::u32::MAX as f64 * 512.0,
            rnd.next_u32() as f64 / std::u32::MAX as f64 * 512.0,
        );
        let r = rnd.next_u32() as f64 / std::u32::MAX as f64 * PI * 2.0;
        let s = rnd.next_u32() as f64 / std::u32::MAX as f64 * 4.0 + 5.0;
        let path = path_transform(
            &path,
            &Matrix::new().rotate(r).scale(s, s).translate(t.0, t.1),
        );

        let color = fill_color::Solid::new(
            [
                rgba(255, 128, 0, 230),
                rgba(0, 255, 128, 230),
                rgba(128, 0, 255, 230),
            ][i % 3],
        );
        context.fill(
            &path,
            &FillStyle {
                color,
                compositor: compositor::SoftLight,
                fill_rule: fill_rule::NonZero,
                pixel: Default::default(),
            },
        );
        let color = fill_color::Solid::new(
            [
                rgba(128, 64, 0, 120),
                rgba(0, 128, 64, 120),
                rgba(64, 0, 128, 120),
            ][i % 3],
        );
        context.stroke(
            &path,
            &FillStyle {
                color,
                compositor: compositor::SrcOver,
                fill_rule: fill_rule::NonZero,
                pixel: Default::default(),
            },
            1.0,
        );
    }
}

fn draw_nanachi<'a>(mut context: ChildContext<'a, Pixel, GenericBuffer<Pixel>>) {
    let (width, height) = (512.0, 512.0);
    let nanachi_path = path_data_notation::parse(
        "
        M 0.41, 0.75
        0.30, 0.74
        0.20, 0.69
        0.18, 0.60
        0.20, 0.52
        0.24, 0.45
        0.33, 0.40
        0.30, 0.25
        0.33, 0.12
        0.36, 0.08
        0.40, 0.15
        0.39, 0.28
        0.38, 0.40
        0.48, 0.38
        0.57, 0.37
        0.57, 0.25
        0.62, 0.10
        0.67, 0.05
        0.70, 0.10
        0.67, 0.25
        0.62, 0.38
        0.74, 0.42
        0.80, 0.50
        0.82, 0.60
        0.78, 0.70
        0.65, 0.76
        0.52, 0.76
        M 0.30, 0.51
        0.40, 0.50
        M 0.33, 0.51
        0.32, 0.55
        0.33, 0.61
        0.37, 0.61
        0.38, 0.55
        0.37, 0.505
        M 0.60, 0.50
        0.70, 0.51
        M 0.63, 0.505
        0.62, 0.55
        0.63, 0.61
        0.67, 0.61
        0.68, 0.55
        0.67, 0.51
        M 0.40, 0.66
        0.45, 0.67
        0.50, 0.66
        0.55, 0.67
        0.60, 0.66
        M 0.45, 0.67
        0.46, 0.70
        0.50, 0.71
        0.54, 0.70
        0.55, 0.67
    ",
    )
    .unwrap();
    let nanachi_path = path_transform(&nanachi_path, &Matrix::new().scale(width, height));

    let moji_path = path_data_notation::parse(
        "
        M 0.30, 0.74
        0.30, 0.86
        0.20, 0.85
        0.17, 0.80
        0.21, 0.77
        0.30, 0.76
        0.42, 0.77
        M 0.24, 0.80
        0.25, 0.81
        M 0.58, 0.76
        0.56, 0.86
        0.44, 0.85
        0.42, 0.82
        0.53, 0.80
        0.65, 0.79
        M 0.52, 0.82
        0.53, 0.83
        M 0.83, 0.78
        0.66, 0.81
        0.72, 0.73
        0.74, 0.80
        0.78, 0.87
    ",
    )
    .unwrap();
    let moji_path = path_transform(&moji_path, &Matrix::new().scale(width, height));

    let shape = path_data_notation::parse(
        "
        M 208.47,399.93
        149.30,394.00
        91.01,362.17
        76.59,307.00
        87.69,260.33
        112.05,219.44
        157.23,199.38
        137.96,127.77
        154.76,54.84
        184.87,32.19
        219.72,73.44
        215.63,144.64
        200.96,197.62
        243.31,178.76
        282.96,181.49
        276.25,125.47
        304.19,43.01
        344.24,15.83
        373.10,48.65
        358.35,132.44
        327.44,189.84
        387.81,202.65
        423.84,249.63
        435.11,308.60
        410.14,368.63
        336.15,404.38
        265.52,405.09
        Z
        M 153.60,373.76
        440.32,378.88
        435.20,424.96
        409.60,460.80
        343.04,445.44
        256.00,455.68
        153.60,450.56
        76.80,440.32
        76.80,399.36
        102.40,384.00
        Z
    ",
    )
    .unwrap();
    let fill_style = FillStyle {
        color: fill_color::Solid::new(rgba(255, 220, 210, 220)),
        compositor: compositor::Screen,
        fill_rule: fill_rule::NonZero,
        pixel: Default::default(),
    };
    context.fill(&shape, &fill_style);

    let fill_style = FillStyle {
        color: fill_color::Solid::new(rgba(64, 8, 8, 255)),
        compositor: compositor::SrcOver,
        fill_rule: fill_rule::NonZero,
        pixel: Default::default(),
    };
    let mut path = nanachi_path;
    path.merge(&moji_path);
    context.stroke(&path, &fill_style, 4.0);
}

fn draw_frame<'a>(mut context: ChildContext<'a, Pixel, GenericBuffer<Pixel>>) {
    let mut rnd = Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
    let size = 8.0;
    let mut rnd = || (1.0 - rnd.next_u32() as f64 / std::u32::MAX as f64 * 2.0) * size;
    let mut builder = PathBuilder::new();
    for i in 0..10 {
        let (dx, dy) = (rnd(), rnd());
        let i = i as f64 / 10.0;
        builder.line_to(10.0 + i * (512.0 - 20.0) + dx, 10.0 + dy);
    }
    for i in 0..10 {
        let (dx, dy) = (rnd(), rnd());
        let i = i as f64 / 10.0;
        builder.line_to(512.0 - 10.0 + dx, 10.0 + i * (512.0 - 20.0) + dy);
    }
    for i in 0..10 {
        let (dx, dy) = (rnd(), rnd());
        let i = i as f64 / 10.0;
        builder.line_to(512.0 - 10.0 - i * (512.0 - 20.0) + dx, 512.0 - 10.0 + dy);
    }
    for i in 0..10 {
        let (dx, dy) = (rnd(), rnd());
        let i = i as f64 / 10.0;
        builder.line_to(10.0 + dx, 512.0 - 10.0 - i * (512.0 - 20.0) + dy);
    }
    builder.close();
    let path = builder.end();
    // let path = smooth(&path);
    context.fill(
        &path,
        &FillStyle::new(
            fill_color::Solid::new(rgba(0, 0, 0, 255)),
            compositor::DstIn,
            fill_rule::EvenOdd,
        ),
    );
}

fn smooth(path: &Path) -> Path {
    path.as_points_list()
        .unwrap()
        .into_iter()
        .map(|mut points| {
            let close = points[0] == *points.last().unwrap();
            if close {
                points.pop();
            }
            Path::from_bezier2_points(&k_curve(points, close, 3))
        })
        .fold(Path::new(vec![]), |mut a, p| {
            a.merge(&p);
            a
        })
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
    Rgba([
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ])
    .into()
}
