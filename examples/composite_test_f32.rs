use nanachi::{
    image::{ImageBuffer, Rgb, Rgba},
    path::Path,
    path_builder::PathBuilder,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
    compositor,
    image_crate_adapter::buffer_rgba_f32_to_rgba_image,
};

fn main() {
    let (width, height) = (320, 320);
    let mut img = ImageBuffer::from_pixel(width, height, rgba(250, 250, 250, 0));

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

    let img = buffer_rgba_f32_to_rgba_image(&img);
    let res = img.save("./composite_test_f32.png");
    println!("save: {:?}", res);
}

fn f<C: compositor::Compositor<Rgba<f32>> + 'static>(img: &mut ImageBuffer<Rgba<f32>, Vec<f32>>, i: usize, c: C) {
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
            (0.1, rgba(255, 0, 0, 150)),
            (0.4, rgba(255, 0, 0, 255)),
            (0.6, rgba(255, 0, 0, 255)),
            (0.9, rgba(255, 255, 0, 255)),
        ]);
    let fc2 = fill_color::LinearGradient::new(
        (-10.0, 0.0),
        (10.0, 0.0),
        vec![
            (0.1, rgba(0, 0, 255, 150)),
            (0.4, rgba(0, 0, 255, 255)),
            (0.6, rgba(0, 0, 255, 255)),
            (0.9, rgba(0, 255, 255, 255)),
        ]);

    let mut img2 = ImageBuffer::from_pixel(60, 60, rgba(250, 250, 250, 0));
    draw_fill(
        &mut img2,
        &path,
        &compositor::SrcOver,
        &fc1,
        Matrix2d::new().translate(20.0, 20.0),
    );
    draw_fill(
        &mut img2,
        &path,
        &c,
        &fc2,
        Matrix2d::new().rotate(std::f64::consts::FRAC_PI_2).translate(20.0, 20.0),
    );
    let x = (60 * (i % 5) + 10) as u32;
    let y = (60 * (i / 5) + 10) as u32;
    for dy in 0..60 {
        for dx in 0..60 {
            img.put_pixel(x+dx, y+dy, *img2.get_pixel(dx, dy));
        }
    }
}

fn draw_fill<C: fill_color::FillColor<Rgba<f32>> + Clone, M: compositor::Compositor<Rgba<f32>> + 'static>(
    img: &mut ImageBuffer<Rgba<f32>, Vec<f32>>,
    path: &Path,
    compositor: &M,
    fill_color: &C,
    matrix: Matrix2d,
) {
    let path = path_transform(path, &matrix);
    let fill_color = nanachi::fill_color::Transform::new(fill_color, matrix);
    nanachi::fill_path::draw_fill(
        img.width() as u32,
        img.height() as u32,
        &path,
        nanachi::fill_rule::NonZero,
        &mut nanachi::writer::img_writer(img, &fill_color, compositor),
        !compositor.keep_dst_on_transparent_src(),
    );
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba<f32> {
    Rgba([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0])
}
