use nanachi::{
    image::{ImageBuffer, Rgb},
    path3::Path,
    path_builder::PathBuilder,
    fill_color,
    path_transform::path_transform,
    matrix::Matrix2d,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    let path = PathBuilder::new()
    .start(10.0, 10.0)
    .line_to(300.0, 10.0)
    // .quad(500.0, 300.0, 300.0, 300.0)
    // .quad(300.0, 200.0, 300.0, 300.0)
    .quad(700.0, 500.0, 300.0, 300.0)
    .arc(200.0, 340.0, 100.0, 0.0, 3.14)
    .arc(200.0, 340.0, 50.0, PI * 3.0, PI * 1.2)
    .line_to(50.0, 250.0)
    .ellipse(100.0, 150.0, 80.0, 50.0, 1.0, PI * 2.7, PI * 1.0)
    .close()
    .end();
    // let path = PathBuilder::new()
    //     .move_to(Point(100.0, 100.0))
    //     .line_to(Point(200.0, 100.0))
    //     .line_to(Point(200.0, 200.0))
    //     .line_to(Point(100.0, 200.0))
    //     .close().end();
    let am = Matrix2d::new()
        .translate(-250.0, -250.0)
        .rotate(0.9)
        .scale(1.0, 0.6)
        .skew_x(-0.1)
        .translate(250.0, 250.0)
    ;
    let path = path_transform(&path, &am);
    {
        let pc = fill_color::LinearGradient::new((200.0, 200.0), (300.0, 400.0), vec![
            (0.0, Rgb([255, 100, 100])),
            (1.0, Rgb([200, 255, 10])),
        ]);
        draw_fill(&mut img, &path, &pc);
    }
    {
        use nanachi::path_outline::{path_outline, Join, Cap};
        let path = Path::new(path_outline(&path, 8.0, &Join::Miter(16.0), &Cap::Round));
        let pc = fill_color::RadialGradient::new((250.0, 200.0), 200.0, vec![
            (0.0, Rgb([255, 255, 255])),
            (0.9, Rgb([200, 10, 10])),
            (1.0, Rgb([10, 10, 255])),
        ]);
        draw_fill(&mut img, &path, &pc);
    }

    let res = img.save("./path3.png");
    println!("save: {:?}", res);
}

fn draw_fill<C: fill_color::FillColor<Rgb<u8>>>(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    path: &Path,
    fill_color: &C,
) {
    let compositor = nanachi::compositor::normal::Normal;
    nanachi::fill_path2::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        nanachi::fill_rule::NonZero,
        &mut nanachi::writer::img_writer(img, fill_color, compositor),
    );
}
