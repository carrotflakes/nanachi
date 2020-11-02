use nanachi::buffer::{Buffer, GenericBuffer};
use nanachi::pixel::{Pixel, Arithmetic, Rgba};

pub fn blur<P: Pixel + Arithmetic, B: Buffer<P>>(buffer: &mut B) {
    let (width, height) = buffer.dimensions();
    let mut tmp = GenericBuffer::from_pixel(width, height, P::zero());
    const SIZE: i32 = 17;
    const HALF: i32 = SIZE / 2;
    let kernel: Vec<f32> = (0..SIZE).map(|i| {
        let f = (i - SIZE / 2) as f32;
        (-f * f / 30.0).exp() * 80.0
    }).collect();
    let a: f32 = 1.0 / kernel.iter().sum::<f32>();

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut p = P::zero();
            for k in 0..SIZE {
                if x - HALF + k < 0 || x - HALF + k >= width as i32 {
                    continue;
                }

                p = p + buffer.get_pixel((x - HALF + k) as u32, y as u32).clone() * kernel[k as usize];
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
    use nanachi::{
        compositor,
        context::{Context, FillStyle},
        fill_color, fill_rule,
        primitives,
    };

    let (width, height) = (512, 512);
    let mut context = Context::from_pixel(width, height, Rgba([1.0, 1.0, 1.0, 1.0])).high_quality();
    let path = primitives::rect(100.0, 100.0, 200.0, 200.0);
    let fill_style = FillStyle::new(
        fill_color::Solid::new(Rgba([1.0, 0.0, 0.0, 0.7])),
        compositor::SrcOver,
        fill_rule::NonZero,
    );
    context.fill(&path, &fill_style);
    let fill_style = FillStyle::new(
        fill_color::Solid::new(Rgba([0.0, 0.0, 1.0, 1.0])),
        compositor::SrcOver,
        fill_rule::NonZero,
    );
    context.stroke(&path, &fill_style, 8.0);

    blur(&mut context.image);
    let img: image::RgbaImage = (&context.image).into();
    img.save("blured_nanachi.png");
}
