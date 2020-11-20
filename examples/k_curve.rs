use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color,
    fill_rule,
    image::RgbaImage,
    k_curve::k_curve,
    matrix::Matrix,
    path::Path,
    path_transform::path_transform,
    pixel::Rgba,
    point::Point,
};

fn main() {
    let (width, height) = (512, 512);
    let mut context = Context::from_pixel(width, height, Rgba([1.0, 1.0, 1.0, 1.0])).high_quality();
    context.flatten_tolerance = 0.1;

    let ps = vec![
        Point(0.2, 0.2),
        Point(0.5, 0.4),
        Point(0.8, 0.2),
        Point(0.8, 0.8),
        Point(0.2, 0.8),
    ];
    let path = Path::from_bezier2_points(&k_curve(ps, true, 3));
    let path = path_transform(&path, &Matrix::new().scale(512.0, 512.0));
    let pc = fill_color::Solid::new(Rgba([0.4, 0.4, 1.0, 1.0]));
    context.fill(
        &path,
        &FillStyle {
            color: pc,
            compositor: compositor::SrcOver,
            fill_rule: fill_rule::NonZero,
            pixel: Default::default(),
        },
    );

    let img: RgbaImage = (&context.image).into();
    let res = img.save("./k_curve.png");
    println!("save: {:?}", res);
}
