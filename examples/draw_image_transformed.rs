use nanachi::{
    compositor, draw_image::draw_image_transformed, image::ImageBuffer, interpolation,
    matrix::Matrix,
};

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("nanachi.png".to_string());
    let src = image::open(file).unwrap().into_rgba();
    let (width, height) = src.dimensions();
    let mut img = ImageBuffer::new(width, height);

    draw_image_transformed(
        &mut img,
        &src,
        (0.0, 0.0, width as f64, height as f64),
        Matrix::new()
            .translate(-200.0, -200.0)
            .scale(1.8, 1.8)
            .rotate(0.2)
            .translate(200.0, 200.0),
        &compositor::DstOver,
        interpolation::Bilinear,
    );

    img.save("./draw_image_transformed.png").unwrap();
}
