use nanachi::{
    buffer::GenericBuffer,
    compositor,
    context::{Context, FillStyle},
    draw_image::draw_image_pixel_perfect,
    fill_color, fill_rule,
    image::RgbaImage,
    matrix::Matrix,
    path_builder::PathBuilder,
    pixel::Pixel,
};

fn main() {
    gen("./composite_test_f32.png", |r: u8, g: u8, b: u8, a: u8| {
        nanachi::pixel::Rgba([
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        ])
    });

    gen(
        "./composite_test_premultiplied_f32.png",
        |r: u8, g: u8, b: u8, a: u8| -> nanachi::pixel::PremultipliedRgba {
            nanachi::pixel::Rgba([
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ])
            .into()
        },
    );

    gen("./composite_test.png", |r: u8, g: u8, b: u8, a: u8| {
        image::Rgba([r, g, b, a])
    });
}

fn gen<P: Pixel>(name: &str, rgba: fn(u8, u8, u8, u8) -> P)
where
    compositor::Basic: compositor::Compositor<P>,
    for<'a> &'a GenericBuffer<P>: Into<RgbaImage>,
{
    let (width, height) = (300, 300);
    let mut img = GenericBuffer::from_pixel(width, height, rgba(250, 250, 250, 0));

    let cs = [
        compositor::Basic::Clear,
        compositor::Basic::Src,
        compositor::Basic::Dst,
        compositor::Basic::SrcOver,
        compositor::Basic::SrcIn,
        compositor::Basic::SrcOut,
        compositor::Basic::SrcAtop,
        compositor::Basic::DstOver,
        compositor::Basic::DstIn,
        compositor::Basic::DstOut,
        compositor::Basic::DstAtop,
        compositor::Basic::Xor,
        compositor::Basic::Add,
        compositor::Basic::Darken,
        compositor::Basic::Lighten,
        compositor::Basic::Multiply,
        compositor::Basic::Screen,
        compositor::Basic::Overlay,
        compositor::Basic::HardLight,
        compositor::Basic::Dodge,
        compositor::Basic::Burn,
        compositor::Basic::SoftLight,
        compositor::Basic::Difference,
        compositor::Basic::Exclusion,
    ];

    for (i, c) in cs.iter().enumerate() {
        f(&mut img, i, c.clone(), rgba);
    }

    let img: RgbaImage = (&img).into();
    let res = img.save(name);
    println!("save: {:?}", res);
}

fn f<P: Pixel>(
    img: &mut GenericBuffer<P>,
    i: usize,
    c: compositor::Basic,
    rgba: fn(u8, u8, u8, u8) -> P,
) where
    compositor::Basic: compositor::Compositor<P>,
{
    let mut pb = PathBuilder::new();
    pb.move_to(-16.0, -20.0);
    pb.line_to(16.0, -20.0);
    pb.line_to(16.0, 20.0);
    pb.line_to(-16.0, 20.0);
    pb.close();
    let path = pb.end();
    let fc1 = fill_color::LinearGradient::new(
        (-16.0, 0.0),
        (16.0, 0.0),
        vec![
            (0.1, rgba(255, 0, 0, 10)),
            (0.4, rgba(255, 0, 0, 255)),
            (0.8, rgba(255, 255, 0, 255)),
        ],
    );
    let fc2 = fill_color::LinearGradient::new(
        (-16.0, 0.0),
        (16.0, 0.0),
        vec![
            (0.1, rgba(0, 0, 255, 10)),
            (0.4, rgba(0, 0, 255, 255)),
            (0.8, rgba(0, 255, 255, 255)),
        ],
    );

    let mut context = Context::from_pixel(60, 60, rgba(250, 250, 250, 0));

    context
        .transformed_context(&Matrix::new().translate(20.0, 20.0))
        .fill(
            &path,
            &FillStyle {
                color: fc1,
                compositor: compositor::Basic::SrcOver,
                fill_rule: fill_rule::EvenOdd,
                pixel: Default::default(),
            },
        );
    context
        .transformed_context(
            &Matrix::new()
                .rotate(90f32.to_radians())
                .translate(20.0, 20.0),
        )
        .fill(
            &path,
            &FillStyle {
                color: fc2,
                compositor: c,
                fill_rule: fill_rule::EvenOdd,
                pixel: Default::default(),
            },
        );
    let x = (60 * (i % 5) + 10) as u32;
    let y = (60 * (i / 5) + 10) as u32;
    draw_image_pixel_perfect(
        img,
        &context.image,
        [x, y],
        [0, 0],
        [50, 50],
        &compositor::Basic::Src,
    );
}
