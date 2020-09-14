use nanachi::{
    image::{ImageBuffer, Rgb, Rgba},
    path3::Path,
    path_builder::PathBuilder,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
};

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([250u8, 250, 250, 0]));

    let mut pb = PathBuilder::new();
    pb.start(0.0, 0.0);
    pb.line_to(40.0, 0.0);
    pb.line_to(40.0, 40.0);
    pb.line_to(0.0, 40.0);
    pb.close();
    let path = pb.end();

    for (i, c) in vec![
        &nanachi::compositor::basic::SrcOver as &dyn nanachi::compositor::Compositor<Rgba<u8>>,
        &nanachi::compositor::basic::SrcIn,
        &nanachi::compositor::basic::SrcOut,
        &nanachi::compositor::basic::SrcAtop,
        &nanachi::compositor::basic::DstOver,
        &nanachi::compositor::basic::DstIn,
        &nanachi::compositor::basic::DstOut,
        &nanachi::compositor::basic::DstAtop,
        &nanachi::compositor::basic::Xor,
    ].into_iter().enumerate() {
        let mut img2 = ImageBuffer::from_pixel(60, 60, Rgba([250u8, 250, 250, 0]));
        draw_fill(
            &mut img2, &path_transform(&path, &Matrix2d::new()),
            &nanachi::compositor::basic::SrcOver,
            &fill_color::Constant::new(Rgba([255, 0, 0, 200])));
        draw_fill(
            &mut img2, &path_transform(&path, &Matrix2d::new().translate(10.0, 10.0)),
            c,
            &fill_color::Constant::new(Rgba([0, 0, 255, 200])));
        let x = (60 * (i % 4) + 10) as u32;
        let y = (60 * (i / 4) + 10) as u32;
        for dy in 0..60 {
            for dx in 0..60 {
                img.put_pixel(x+dx, y+dy, *img2.get_pixel(dx, dy));
            }
        }
    }

    let res = img.save("./composite_test.png");
    println!("save: {:?}", res);
}

fn draw_fill<C: fill_color::FillColor<Rgba<u8>>>(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    path: &Path,
    compositor: &dyn nanachi::compositor::Compositor<Rgba<u8>>,
    fill_color: &C,
) {
    nanachi::fill_path::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        nanachi::fill_rule::NonZero,
        &mut nanachi::writer::img_writer(img, fill_color, compositor),
    );
}
