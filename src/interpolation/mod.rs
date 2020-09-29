use crate::buffer::Buffer;
use crate::pixel::Pixel;

pub trait Interpolation<P: Pixel, B: Buffer<P>> {
    fn interpolate(&self, buffer: &B, x: f64, y: f64) -> P;
}

#[derive(Clone)]
pub struct NearestNeighbor;

impl<P: Pixel, B: Buffer<P>> Interpolation<P, B> for NearestNeighbor {
    fn interpolate(&self, buffer: &B, x: f64, y: f64) -> P {
        let (width, height) = buffer.dimensions();
        buffer
            .get_pixel(
                x.rem_euclid(width as f64).floor() as u32,
                y.rem_euclid(height as f64).floor() as u32,
            )
            .clone()
    }
}

#[derive(Clone)]
pub struct Bilinear;

impl<P: Pixel, B: Buffer<P>> Interpolation<P, B> for Bilinear {
    fn interpolate(&self, buffer: &B, x: f64, y: f64) -> P {
        let (width, height) = buffer.dimensions();
        let x = x.rem_euclid(width as f64);
        let y = y.rem_euclid(height as f64);
        let x2 = (x as u32 + 1) % width;
        let y2 = (y as u32 + 1) % height;
        buffer
            .get_pixel(x as u32, y as u32)
            .lerp(buffer.get_pixel(x2, y as u32), x.fract())
            .lerp(
                &buffer
                    .get_pixel(x as u32, y2)
                    .lerp(buffer.get_pixel(x2, y2), x.fract()),
                y.fract(),
            )
    }
}
