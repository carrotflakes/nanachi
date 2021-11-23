use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    image::{ImageBuffer, Rgba},
    matrix::Matrix,
    path_builder::PathBuilder,
    primitives,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);

    let mut img = ImageBuffer::from_pixel(width, height, Rgba([0u8, 0, 0, 0]));
    let mut context = Context::from_image(&mut img);

    let mut builder = PathBuilder::new();
    builder.move_to(10.0, 40.0);
    builder.line_to(100.0, 40.0);
    builder.cubic(170.0, 100.0, 200.0, 10.0, 300.0, 40.0);
    builder.line_to(300.0, 40.0);
    builder.quad(700.0, 500.0, 300.0, 300.0);
    builder.arc(200.0, 340.0, 100.0, 0.0, 3.14);
    builder.arc(200.0, 340.0, 50.0, PI * 3.0, PI * 1.2);
    builder.line_to(50.0, 250.0);
    builder.ellipse(100.0, 150.0, 80.0, 50.0, 1.0, PI * 2.7, PI * 1.0);
    builder.close();
    builder.arc(280.0, 220.0, 40.0, PI * 2.0, 0.0);
    builder.close();
    let path = builder.end();

    let t = std::time::Instant::now();
    context.transformed_context(
        &Matrix::new()
            .translate(50.0, 50.0)
    ).fill(
        &primitives::ngon(0.0, 0.0, 5, 40.0),
        &FillStyle {
            color: fill_color::Solid::new(Rgba([200, 200, 0, 255])),
            fill_rule: fill_rule::NonZero,
            compositor: compositor::SrcOver,
            pixel: Default::default(),
        },
    );

    let mut context = context.transformed_context(
        &Matrix::new()
            .translate(-250.0, -250.0)
            .rotate(0.9)
            .scale(1.0, 0.6)
            .skew_x(-0.1)
            .translate(250.0, 280.0),
    );
    {
        let color = fill_color::LinearGradient::new(
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
                color,
                fill_rule: fill_rule::NonZero,
                compositor: compositor::SrcOver,
                pixel: Default::default(),
            },
        );
    }
    {
        let color = fill_color::RadialGradient::new(
            (250.0, 220.0),
            220.0,
            vec![
                (0.0, Rgba([255, 255, 255, 255])),
                (0.9, Rgba([200, 10, 10, 255])),
                (1.0, Rgba([10, 10, 255, 10])),
            ],
        );
        context.stroke(
            &path,
            &FillStyle {
                color,
                fill_rule: fill_rule::NonZero,
                compositor: compositor::SrcOver,
                pixel: Default::default(),
            },
            8.0,
        );
    }
    dbg!(t.elapsed());

    let res = img.save("./context.png");
    println!("save: {:?}", res);
}
