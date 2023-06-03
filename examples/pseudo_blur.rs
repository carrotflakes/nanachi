use nanachi::buffer::{Buffer, GenericBuffer};
use nanachi::pixel::{Arithmetic, Pixel, Rgba};

pub fn blur<P: Pixel + Arithmetic, B: Buffer<P>>(buffer: &mut B) {
    let (width, height) = buffer.dimensions();
    let mut tmp = GenericBuffer::from_pixel(width, height, P::zero());
    const SIZE: i32 = 51;
    const HALF: i32 = SIZE / 2;
    let sigma = 10.0;
    let denominator = -2.0 * sigma * sigma;
    let kernel: Vec<f32> = (0..SIZE)
        .map(|i| {
            let f = (i - SIZE / 2) as f32;
            (f * f / denominator).exp()
        })
        .collect();
    let a: f32 = 1.0 / kernel.iter().sum::<f32>();

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut p = P::zero();
            for k in 0..SIZE {
                if x - HALF + k < 0 || x - HALF + k >= width as i32 {
                    continue;
                }

                p = p + buffer.get_pixel((x - HALF + k) as u32, y as u32).clone()
                    * kernel[k as usize];
            }
            tmp.put_pixel(x as u32, y as u32, p * a);
        }
    }

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut p = P::zero();
            for k in 0..SIZE {
                if y - HALF + k < 0 || y - HALF + k >= height as i32 {
                    continue;
                }

                p = p + tmp.get_pixel(x as u32, (y - HALF + k) as u32).clone() * kernel[k as usize];
            }
            buffer.put_pixel(x as u32, y as u32, p * a);
        }
    }
}

fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("nanachi.png".to_string());
    let src = image::open(file).unwrap().into_rgba32f();
    let (width, height) = src.dimensions();
    let mut img = GenericBuffer::from_pixel(width, height, Rgba::zero());
    for y in 0..height {
        for x in 0..width {
            let p = src.get_pixel(x, y).0;
            img.put_pixel(x, y, Rgba(p));
        }
    }

    let t = std::time::Instant::now();
    blur(&mut img);
    dbg!(t.elapsed());
    let img: image::RgbaImage = (&img).into();
    img.save("pseudo_blur.png").unwrap();
}
