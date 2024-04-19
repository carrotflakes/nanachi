use image::RgbaImage;
use nanachi::{context::Context, fill_color, pixel::Rgba};

fn main() {
    let (width, height) = (512, 512);
    let mut context = Context::from_pixel(width, height, rgba(255, 255, 255, 255)).high_quality();
    context.clear(&fill_color::ConicGradient::new(
        (width as f32 / 2.0, height as f32 / 2.0),
        0.1,
        vec![(0.0, rgba(255, 0, 0, 255)), (1.0, rgba(0, 255, 255, 255))],
    ));

    let img: RgbaImage = (&context.image).into();
    img.save("./conic_grad.png").unwrap();
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
    Rgba([
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ])
    .into()
}
