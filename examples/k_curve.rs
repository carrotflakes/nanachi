use nanachi::{
    image::{ImageBuffer, Rgb},
    path3::Path,
    point::Point,
    position_color,
    path_transform::path_transform,
    matrix::Matrix2d,
    k_curve::k_curve,
};

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    let ps = vec![
        Point(0.2, 0.2),
        Point(0.8, 0.2),
        Point(0.8, 0.8),
        Point(0.2, 0.8),
    ];
    let path = Path::from_bezier2_points(&k_curve(ps, true, 3));
    let path = path_transform(&path, &Matrix2d::new().scale(512.0, 512.0));
    let pc = position_color::Constant::new(Rgb([100, 100, 250]));
    draw_fill(&mut img, &path, &pc, 1.0);

    let res = img.save("./k_curve.png");
    println!("save: {:?}", res);
}

fn draw_fill<X, C: nanachi::position_color::PositionColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    path: &Path,
    position_color: &C,
    alpha: f64,
) where
    X: image::Pixel<Subpixel = u8> + 'static,
{
    nanachi::fill_path2::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        &mut nanachi::writer::alpha_blend2(img, position_color, nanachi::writer::FillRule::NonZero, alpha),
    );
}
