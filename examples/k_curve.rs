use nanachi::{
    image::{ImageBuffer, Rgb},
    path::Path,
    point::Point,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
    k_curve::k_curve,
};

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    let ps = vec![
        Point(0.2, 0.2),
        Point(0.5, 0.4),
        Point(0.8, 0.2),
        Point(0.8, 0.8),
        Point(0.2, 0.8),
    ];
    let path = Path::from_bezier2_points(&k_curve(ps, true, 3));
    let path = path_transform(&path, &Matrix2d::new().scale(512.0, 512.0));
    let pc = fill_color::Solid::new(Rgb([100, 100, 250]));
    draw_fill(&mut img, &path, &pc);

    let res = img.save("./k_curve.png");
    println!("save: {:?}", res);
}

fn draw_fill<C: fill_color::FillColor<Rgb<u8>>>(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    path: &Path,
    fill_color: &C,
) {
    nanachi::fill_path::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        nanachi::fill_rule::NonZero,
        &mut nanachi::writer::img_writer(img, fill_color, &nanachi::compositor::SrcOver),
        false,
    );
}
