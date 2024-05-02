use nanachi::{
    compositor, contrib::draw_image_transformed::draw_image_transformed, image::ImageBuffer,
    interpolation, matrix::Matrix,
};

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("nanachi.png".to_string());
    let src = image::open(file).unwrap().into_rgba8();
    let (width, height) = src.dimensions();
    let mut img = ImageBuffer::new(width, height);

    draw_image_transformed(
        &mut img,
        &src,
        [110.0, 110.0, width as f32 - 110.0, height as f32 - 110.0],
        Matrix::new()
            .scale(0.8, 0.8)
            .rotate(0.2)
            .translate(10.0, 10.0),
        &compositor::SrcOver,
        interpolation::Bilinear,
    );

    img.save("./draw_image_transformed.png").unwrap();
}
