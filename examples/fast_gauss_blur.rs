use nanachi::buffer::{Buffer, GenericBuffer};
use nanachi::contrib::gauss_blur;
use nanachi::pixel::{Arithmetic, Rgba};

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("nanachi.png".to_string());
    let src = image::open(file).unwrap().into_rgba();
    let (width, height) = src.dimensions();
    let mut img = GenericBuffer::from_pixel(width, height, Rgba::zero());
    for y in 0..height {
        for x in 0..width {
            let p = src.get_pixel(x, y).0;
            img.put_pixel(
                x,
                y,
                Rgba([
                    p[0] as f32 / 255.0,
                    p[1] as f32 / 255.0,
                    p[2] as f32 / 255.0,
                    p[3] as f32 / 255.0,
                ]),
            );
        }
    }

    let mut tmp = GenericBuffer::from_pixel(width, height, Rgba::zero());
    let t = std::time::Instant::now();
    gauss_blur::gauss_blur(
        &mut img,
        &mut tmp,
        10.0,
        &gauss_blur::Extrapolation::ExtendEdge,
        3,
    );
    // box_blur(&mut img, &mut tmp, 10);
    dbg!(t.elapsed());
    let img: image::RgbaImage = (&img).into();
    img.save("fast_gauss_blur.png").unwrap();
}
