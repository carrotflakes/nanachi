use image::RgbaImage;
use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    pixel::Rgba,
    primitives,
};

fn main() {
    let (width, height) = (200, 200);

    let mut context = Context::from_pixel(width, height, Rgba([0.0, 0.0, 0.0, 1.0])).high_quality();

    context.fill(
        &primitives::rect(20.0, 20.0, 200.0 - 40.0, 200.0 - 40.0),
        &FillStyle::new(
            fill_color::Solid::new(Rgba([1.0, 1.0, 0.0, 0.7])),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::circle(60.0, 60.0, 50.0),
        &FillStyle::new(
            fill_color::Solid::new(Rgba([1.0, 0.0, 0.0, 0.7])),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::triangle(140.0, 60.0, 50.0),
        &FillStyle::new(
            fill_color::Solid::new(Rgba([0.0, 1.0, 0.0, 0.7])),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::ngon(60.0, 140.0, 4, 50.0),
        &FillStyle::new(
            fill_color::Solid::new(Rgba([0.0, 0.0, 1.0, 0.7])),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::ngon(140.0, 140.0, 5, 50.0),
        &FillStyle::new(
            fill_color::Solid::new(Rgba([0.0, 1.0, 1.0, 0.7])),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    let img: RgbaImage = (&context.image).into();
    img.save("./primitive_shapes.png").unwrap();
}
