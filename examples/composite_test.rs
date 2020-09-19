use nanachi::{
    image::{ImageBuffer, Rgb, Rgba},
    path::Path,
    path_builder::PathBuilder,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
};

fn main() {
    let (width, height) = (320, 320);
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([250u8, 250, 250, 0]));

    #[allow(arithmetic_overflow)]
    let mut i = 0 - 1;
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Clear);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Src);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Dst);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::SrcOver);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::SrcIn);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::SrcOut);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::SrcAtop);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::DstOver);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::DstIn);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::DstOut);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::DstAtop);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Xor);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Add);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Darken);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Lighten);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Multiply);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Screen);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Overlay);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::HardLight);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Dodge);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Burn);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::SoftLight);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Difference);
    f(&mut img, {i += 1; i}, nanachi::compositor::basic::Exclusion);

    let res = img.save("./composite_test.png");
    println!("save: {:?}", res);
}

fn f<C: nanachi::compositor::Compositor<Rgba<u8>> + 'static>(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, i: usize, c: C) {
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

    let mut img2 = ImageBuffer::from_pixel(60, 60, Rgba([250u8, 250, 250, 0]));
    draw_fill(
        &mut img2,
        &path,
        &nanachi::compositor::basic::SrcOver,
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

fn draw_fill<C: fill_color::FillColor<Rgba<u8>> + Clone, M: nanachi::compositor::Compositor<Rgba<u8>> + 'static>(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
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
    );
}
