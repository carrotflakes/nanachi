use nanachi::{
    image::{ImageBuffer, Rgb, Rgba},
    path_builder::PathBuilder,
    fill_color,
    matrix::Matrix,
    compositor,
    context::{Context, FillStyle},
    fill_rule,
    draw_image::draw_image_pixel_perfect,
};

fn main() {
    let (width, height) = (320, 320);
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([250u8, 250, 250, 0]));

    #[allow(arithmetic_overflow)]
    let mut i = 0 - 1;
    f(&mut img, {i += 1; i}, compositor::Clear);
    f(&mut img, {i += 1; i}, compositor::Src);
    f(&mut img, {i += 1; i}, compositor::Dst);
    f(&mut img, {i += 1; i}, compositor::SrcOver);
    f(&mut img, {i += 1; i}, compositor::SrcIn);
    f(&mut img, {i += 1; i}, compositor::SrcOut);
    f(&mut img, {i += 1; i}, compositor::SrcAtop);
    f(&mut img, {i += 1; i}, compositor::DstOver);
    f(&mut img, {i += 1; i}, compositor::DstIn);
    f(&mut img, {i += 1; i}, compositor::DstOut);
    f(&mut img, {i += 1; i}, compositor::DstAtop);
    f(&mut img, {i += 1; i}, compositor::Xor);
    f(&mut img, {i += 1; i}, compositor::Add);
    f(&mut img, {i += 1; i}, compositor::Darken);
    f(&mut img, {i += 1; i}, compositor::Lighten);
    f(&mut img, {i += 1; i}, compositor::Multiply);
    f(&mut img, {i += 1; i}, compositor::Screen);
    f(&mut img, {i += 1; i}, compositor::Overlay);
    f(&mut img, {i += 1; i}, compositor::HardLight);
    f(&mut img, {i += 1; i}, compositor::Dodge);
    f(&mut img, {i += 1; i}, compositor::Burn);
    f(&mut img, {i += 1; i}, compositor::SoftLight);
    f(&mut img, {i += 1; i}, compositor::Difference);
    f(&mut img, {i += 1; i}, compositor::Exclusion);

    let res = img.save("./composite_test.png");
    println!("save: {:?}", res);
}

fn f<C: compositor::Compositor<Rgba<u8>> + 'static>(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, i: usize, c: C) {
    let mut pb = PathBuilder::new();
    pb.move_to(-10.0, -20.0);
    pb.line_to(10.0, -20.0);
    pb.line_to(10.0, 20.0);
    pb.line_to(-10.0, 20.0);
    pb.close();
    let path = pb.end();
    let fc1 = fill_color::LinearGradient::new(
        (-10.0, 0.0),
        (10.0, 0.0),
        vec![
            (0.1, Rgba([255, 0, 0, 150])),
            (0.4, Rgba([255, 0, 0, 255])),
            (0.6, Rgba([255, 0, 0, 255])),
            (0.9, Rgba([255, 255, 0, 255])),
        ]);
    let fc2 = fill_color::LinearGradient::new(
        (-10.0, 0.0),
        (10.0, 0.0),
        vec![
            (0.1, Rgba([0, 0, 255, 150])),
            (0.4, Rgba([0, 0, 255, 255])),
            (0.6, Rgba([0, 0, 255, 255])),
            (0.9, Rgba([0, 255, 255, 255])),
        ]);

    let mut context = Context::from_pixel(60, 60, Rgba([250, 250, 250, 0]));

    context.transformed_context(&Matrix::new().translate(20.0, 20.0))
    .fill(&path, &FillStyle{
        color: fc1,
        compositor: compositor::SrcOver,
        fill_rule: fill_rule::EvenOdd,
        pixel: Default::default(),
    });
    context.transformed_context(&Matrix::new().rotate(90f64.to_radians()).translate(20.0, 20.0))
    .fill(&path, &FillStyle{
        color: fc2,
        compositor: c,
        fill_rule: fill_rule::EvenOdd,
        pixel: Default::default(),
    });
    let x = (60 * (i % 5) + 10) as u32;
    let y = (60 * (i / 5) + 10) as u32;
    draw_image_pixel_perfect(img, &context.image, (x, y), (0, 0), (60, 60), &compositor::Src);
}
