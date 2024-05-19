use nanachi::{
    buffer::GenericBuffer,
    compositor,
    context::{Context, FillStyle},
    draw_image::draw_image_pixel_perfect,
    fill_color, fill_rule,
    image::RgbaImage,
    matrix::Matrix,
    path_builder::PathBuilder,
    pixel::{PremultipliedRgba, Rgba},
};

fn main() {
    let (width, height) = (320, 320);
    let mut img = GenericBuffer::from_pixel(width, height, rgba(250, 250, 250, 0));

    let mut i = 0;
    let mut inc = || {
        i += 1;
        i - 1
    };
    f(&mut img, inc(), compositor::Clear);
    f(&mut img, inc(), compositor::Src);
    f(&mut img, inc(), compositor::Dst);
    f(&mut img, inc(), compositor::SrcOver);
    f(&mut img, inc(), compositor::SrcIn);
    f(&mut img, inc(), compositor::SrcOut);
    f(&mut img, inc(), compositor::SrcAtop);
    f(&mut img, inc(), compositor::DstOver);
    f(&mut img, inc(), compositor::DstIn);
    f(&mut img, inc(), compositor::DstOut);
    f(&mut img, inc(), compositor::DstAtop);
    f(&mut img, inc(), compositor::Xor);
    f(&mut img, inc(), compositor::Add);
    f(&mut img, inc(), compositor::Darken);
    f(&mut img, inc(), compositor::Lighten);
    f(&mut img, inc(), compositor::Multiply);
    f(&mut img, inc(), compositor::Screen);
    f(&mut img, inc(), compositor::Overlay);
    f(&mut img, inc(), compositor::HardLight);
    f(&mut img, inc(), compositor::Dodge);
    f(&mut img, inc(), compositor::Burn);
    f(&mut img, inc(), compositor::SoftLight);
    f(&mut img, inc(), compositor::Difference);
    f(&mut img, inc(), compositor::Exclusion);

    let img: RgbaImage = (&img).into();
    let res = img.save("./composite_test_premultiplied_f32.png");
    println!("save: {:?}", res);
}

fn f<C: compositor::Compositor<PremultipliedRgba> + 'static>(
    img: &mut GenericBuffer<PremultipliedRgba>,
    i: usize,
    c: C,
) {
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
                compositor: compositor::SrcOver,
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
        [60, 60],
        &compositor::Src,
    );
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> PremultipliedRgba {
    Rgba([
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ])
    .into()
}
