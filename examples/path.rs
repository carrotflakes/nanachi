use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    image::{ImageBuffer, Rgb, Rgba},
    interpolation,
    matrix::Matrix,
    path::Path,
    path_builder::PathBuilder,
    path_transform::path_transform,
};
use std::f32::consts::PI;

fn main() {
    let (width, height) = (512, 512);

    let bg_image = {
        let mut ctx = Context::from_pixel(10, 10, Rgba([0u8, 0, 0, 255])).high_quality();
        let mut pb = PathBuilder::new();
        pb.arc(5.0, 5.0, 4.0, 0.0, PI * 2.0);
        let path = pb.end();
        ctx.fill(
            &path,
            &FillStyle {
                color: fill_color::Solid::new(Rgba([80, 200, 255, 50])),
                compositor: compositor::SrcOver,
                fill_rule: fill_rule::NonZero,
                pixel: Default::default(),
            },
        );
        ctx.image
    };
    let pattern = fill_color::Pattern::new(&bg_image, interpolation::Bilinear, Default::default());
    let bg_fill_color =
        fill_color::Transform::new(&pattern, Matrix::new().rotate(PI * 0.25).scale(2.0, 2.0));
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        use nanachi::fill_color::FillColor;
        bg_fill_color.fill_color([x as f32, y as f32])
    });
    let mut context = Context::from_image(&mut img).high_quality();

    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 40.0);
    pb.line_to(100.0, 40.0);
    pb.cubic(170.0, 100.0, 200.0, 10.0, 300.0, 40.0);
    pb.line_to(300.0, 40.0);
    // .quad(500.0, 300.0, 300.0, 300.0);
    // .quad(300.0, 200.0, 300.0, 300.0);
    pb.quad(700.0, 500.0, 300.0, 300.0);
    pb.arc(200.0, 340.0, 100.0, 0.0, 3.14);
    pb.arc(200.0, 340.0, 50.0, PI * 3.0, PI * 1.2);
    pb.line_to(50.0, 250.0);
    pb.ellipse(100.0, 150.0, 80.0, 50.0, 1.0, PI * 2.7, PI * 1.0);
    pb.close();
    pb.arc(280.0, 220.0, 40.0, PI * 2.0, 0.0);
    pb.close();
    // pb.move_to(100.0, 100.0);
    // pb.line_to(200.0, 100.0);
    // pb.line_to(200.0, 200.0);
    // pb.line_to(100.0, 200.0);
    // pb.close();
    let path = pb.end();

    let am = Matrix::new()
        .translate(-250.0, -250.0)
        // .scale(-1.0, -1.0)
        .rotate(0.9)
        .scale(1.0, 0.6)
        .skew_x(-0.1)
        .translate(250.0, 280.0);
    let path = path_transform(&path, &am);
    let path = nanachi::path_flatten::path_flatten_only_cubic(&path, 0.5);
    let t = std::time::Instant::now();
    {
        let pc = fill_color::LinearGradient::new(
            (200.0, 200.0),
            (300.0, 430.0),
            vec![
                (0.0, Rgba([255, 100, 100, 100])),
                (1.0, Rgba([200, 255, 10, 255])),
            ],
        );
        context.fill(
            &path,
            &FillStyle {
                color: pc,
                compositor: compositor::SrcOver,
                fill_rule: fill_rule::NonZero,
                pixel: Default::default(),
            },
        );
    }
    {
        use nanachi::path_outline::{path_outline, Cap, Join};
        let path = path_outline(&path, 8.0, &Join::Round, &Cap::Round);
        let pc = fill_color::RadialGradient::new(
            (250.0, 220.0),
            220.0,
            vec![
                (0.0, Rgba([255, 255, 255, 255])),
                (0.9, Rgba([200, 10, 10, 255])),
                (1.0, Rgba([10, 10, 255, 100])),
            ],
        );
        context.fill(
            &path,
            &FillStyle {
                color: pc,
                compositor: compositor::SrcOver,
                fill_rule: fill_rule::NonZero,
                pixel: Default::default(),
            },
        );
    }
    dbg!(t.elapsed());

    let res = img.save("./path.png");
    println!("save: {:?}", res);
}
