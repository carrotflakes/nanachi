use image::RgbaImage;
use nanachi::{buffer::{Buffer, GenericBuffer}, compositor, context::{Context, FillStyle}, fill_color, fill_rule, interpolation, matrix::Matrix, pixel::Rgba, primitives};

fn main() {
    let (width, height) = (200, 200);

    let mut context = Context::from_pixel(width, height, Rgba([0.0, 0.0, 0.0, 1.0])).high_quality();

    context.fill(
        &primitives::rect(40.0, 40.0, 200.0 - 80.0, 200.0 - 80.0),
        &FillStyle::new(
            fill_color::Pattern::new(
                &{
                    let pixel = Rgba([1.0, 1.0, 0.0, 1.0]);
                    let mut buffer = GenericBuffer::from_pixel(3, 3, Rgba([0.0, 0.0, 0.0, 0.0]));
                    buffer.put_pixel(0, 0, pixel);
                    buffer
                },
                interpolation::NearestNeighbor,
                Matrix::new(),
            ),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::circle(60.0, 60.0, 50.0),
        &FillStyle::new(
            fill_color::Pattern::new(
                &{
                    let pixel = Rgba([1.0, 0.0, 0.0, 1.0]);
                    let mut buffer = GenericBuffer::from_pixel(3, 3, Rgba([0.0, 0.0, 0.0, 0.0]));
                    buffer.put_pixel(1, 0, pixel);
                    buffer.put_pixel(0, 1, pixel);
                    buffer.put_pixel(1, 1, pixel);
                    buffer.put_pixel(2, 1, pixel);
                    buffer.put_pixel(1, 2, pixel);
                    buffer
                },
                interpolation::NearestNeighbor,
                Matrix::new(),
            ),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::triangle(140.0, 60.0, 50.0),
        &FillStyle::new(
            fill_color::Pattern::new(
                &{
                    let pixel = Rgba([0.0, 1.0, 0.0, 1.0]);
                    let mut buffer = GenericBuffer::from_pixel(4, 4, Rgba([0.0, 0.0, 0.0, 0.0]));
                    buffer.put_pixel(0, 0, pixel);
                    buffer.put_pixel(1, 1, pixel);
                    buffer.put_pixel(2, 2, pixel);
                    buffer.put_pixel(3, 3, pixel);
                    buffer
                },
                interpolation::NearestNeighbor,
                Matrix::new(),
            ),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::ngon(60.0, 140.0, 4, 50.0),
        &FillStyle::new(
            fill_color::Pattern::new(
                &{
                    let pixel = Rgba([1.0, 0.0, 1.0, 1.0]);
                    let mut buffer = GenericBuffer::from_pixel(2, 2, Rgba([0.0, 0.0, 0.0, 0.0]));
                    buffer.put_pixel(0, 0, pixel);
                    buffer.put_pixel(1, 0, pixel);
                    buffer
                },
                interpolation::NearestNeighbor,
                Matrix::new(),
            ),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    context.fill(
        &primitives::ngon(140.0, 140.0, 5, 50.0),
        &FillStyle::new(
            fill_color::Pattern::new(
                &{
                    let mut context = Context::from_pixel(20, 20, Rgba([0.0, 0.0, 0.0, 0.0])).high_quality();
                    let pixel = Rgba([0.0, 1.0, 1.0, 0.7]);
                    context.fill(
                        &primitives::circle(10.0, 10.0, 8.0),
                        &FillStyle::new(
                            fill_color::Solid::new(pixel),
                            compositor::SrcOver,
                            fill_rule::NonZero,
                        ),
                    );
                    context.image
                },
                interpolation::NearestNeighbor,
                Matrix::new().scale(2.0, 2.0),
            ),
            compositor::SrcOver,
            fill_rule::NonZero,
        ),
    );

    let img: RgbaImage = (&context.image).into();
    img.save("./pattern.png").unwrap();
}
