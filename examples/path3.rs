use nanachi::{
    image::{ImageBuffer, Rgb, Rgba},
    path::Path,
    path_builder::PathBuilder,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);

    let bg_image = {
        let mut img = ImageBuffer::from_pixel(10, 10, Rgba([0u8, 0, 0, 255]));
        let mut pb = PathBuilder::new();
        pb.arc(5.0, 5.0, 4.0, 0.0, PI * 2.0);
        let path = pb.end();
        let pc = fill_color::Constant::new(Rgba([80, 200, 255, 50]));
        draw_fill(&mut img, &path, nanachi::compositor::basic::SrcOver, &pc);
        img
    };
    let pattern = fill_color::Pattern::new(&bg_image);
    let bg_fill_color = fill_color::Transform::new(
        &pattern,
        Matrix2d::new().rotate(PI * 0.25).scale(2.0, 2.0));
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        use nanachi::fill_color::FillColor;
        bg_fill_color.fill_color(x as f64, y as f64)
    });

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

    let am = Matrix2d::new()
        .translate(-250.0, -250.0)
        .rotate(0.9)
        .scale(1.0, 0.6)
        .skew_x(-0.1)
        .translate(250.0, 280.0)
    ;
    let path = path_transform(&path, &am);
    let path = nanachi::path_flatten::path_flatten_only_cubic(&path, 0.5);
    let t = std::time::Instant::now();
    {
        let pc = fill_color::LinearGradient::new((200.0, 200.0), (300.0, 430.0), vec![
            (0.0, Rgba([255, 100, 100, 100])),
            (1.0, Rgba([200, 255, 10, 255])),
        ]);
        draw_fill(&mut img, &path, nanachi::compositor::basic::SrcOver, &pc);
    }
    {
        use nanachi::path_outline::{path_outline, Join, Cap};
        let path = path_outline(&path, 8.0, &Join::Round, &Cap::Round);
        let pc = fill_color::RadialGradient::new((250.0, 220.0), 220.0, vec![
            (0.0, Rgba([255, 255, 255, 255])),
            (0.9, Rgba([200, 10, 10, 255])),
            (1.0, Rgba([10, 10, 255, 100])),
        ]);
        draw_fill(&mut img, &path, nanachi::compositor::basic::SrcOver, &pc);
    }
    dbg!(t.elapsed());

    let res = img.save("./path3.png");
    println!("save: {:?}", res);
}

fn draw_fill<C: fill_color::FillColor<Rgba<u8>>, M: nanachi::compositor::Compositor<Rgba<u8>> + 'static>(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    path: &Path,
    compositor: M,
    fill_color: &C,
) {
    nanachi::fill_path::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        nanachi::fill_rule::NonZero,
        &mut nanachi::writer::img_writer(img, fill_color, &compositor),
    );
}
