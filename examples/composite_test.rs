use nanachi::{
    image::{ImageBuffer, Rgb, Rgba},
    path::Path,
    path_builder::PathBuilder,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
};

fn main() {
    let (width, height) = (250, 250);
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([250u8, 250, 250, 0]));

    fn f<C: nanachi::compositor::Compositor<Rgba<u8>> + 'static>(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, i: usize, c: C) {
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, 0.0);
        pb.line_to(40.0, 0.0);
        pb.line_to(40.0, 40.0);
        pb.line_to(0.0, 40.0);
        pb.close();
        let path = pb.end();

        let mut img2 = ImageBuffer::from_pixel(60, 60, Rgba([250u8, 250, 250, 0]));
        draw_fill(
            &mut img2, &path_transform(&path, &Matrix2d::new()),
            &nanachi::compositor::basic::SrcOver,
            &fill_color::Constant::new(Rgba([255, 0, 0, 200])));
        draw_fill(
            &mut img2, &path_transform(&path, &Matrix2d::new().translate(10.0, 10.0)),
            &c,
            &fill_color::Constant::new(Rgba([0, 0, 255, 150])));
        let x = (60 * (i % 4) + 10) as u32;
        let y = (60 * (i / 4) + 10) as u32;
        for dy in 0..60 {
            for dx in 0..60 {
                img.put_pixel(x+dx, y+dy, *img2.get_pixel(dx, dy));
            }
        }
    }
    f(&mut img, 0, nanachi::compositor::basic::SrcOver);
    f(&mut img, 1, nanachi::compositor::basic::SrcIn);
    f(&mut img, 2, nanachi::compositor::basic::SrcOut);
    f(&mut img, 3, nanachi::compositor::basic::SrcAtop);
    f(&mut img, 4, nanachi::compositor::basic::DstOver);
    f(&mut img, 5, nanachi::compositor::basic::DstIn);
    f(&mut img, 6, nanachi::compositor::basic::DstOut);
    f(&mut img, 7, nanachi::compositor::basic::DstAtop);
    f(&mut img, 8, nanachi::compositor::basic::Xor);
    f(&mut img, 9, nanachi::compositor::basic::Add);
    f(&mut img, 10, nanachi::compositor::basic::Darken);
    f(&mut img, 11, nanachi::compositor::basic::Lighten);
    f(&mut img, 12, nanachi::compositor::basic::Multiply);
    f(&mut img, 13, nanachi::compositor::basic::Screen);

    let res = img.save("./composite_test.png");
    println!("save: {:?}", res);
}

fn draw_fill<C: fill_color::FillColor<Rgba<u8>>, M: nanachi::compositor::Compositor<Rgba<u8>> + 'static>(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    path: &Path,
    compositor: &M,
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
